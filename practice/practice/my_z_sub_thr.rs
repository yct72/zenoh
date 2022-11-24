use zenoh::config::Config;
use clap::{App, Arg};
// use zenoh::prelude::sync::*;
use zenoh::prelude::r#async::*;

fn increment_x(x: &mut usize) {
    println!("x={}", x);
    *x = *x + 1;
}

#[async_std::main]
async fn main() {
    /* 
        Send and Sync: https://doc.rust-lang.org/nomicon/send-and-sync.html
        A type is Send if it is safe to send it to another thread.
        A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).
    */
    let (config, payload, number, loops) = parse_args();
    let key_expr = "test/thr";

    println!("Opening Zenoh session...");
    let session = zenoh::open(config).res().await.unwrap();
    
    println!("Creating Subscriber...");
    
    

    
    let subscriber = session
        .declare_subscriber(key_expr)
        .res().await.unwrap();

    let mut start_time = std::time::Instant::now();
    let mut avg_thr: f64 = 0.0;

    for l in 0..loops {
        for x in 0..number {
            let sample = subscriber.recv_async().await.unwrap();
            // let sample = sample.unwrap();
            // println!(">> x = {}, [Subscriber] Received ('{}': '')",
            //     x, sample.key_expr.as_str());
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        let thr = number as f64 / elapsed;
        // println!("[Loop {}]", l);
        // println!("- Payload: {} bytes", payload);
        // println!("- Throughput: {} msg/s", thr);
        // println!("------------------------------------------");

        avg_thr += thr / loops as f64;
    }
    
    println!("hi");
    println!("*****************************************************************");
    println!("Zenoh Subscription Throughput Test");
    println!("- Payload: {} bytes", payload);
    println!("- Average throughput for {} loops: {} msg/s", loops, avg_thr);
    println!("*****************************************************************");

    
} 



fn parse_args() -> (Config, usize, usize, usize) {
    
    let args = App::new("Zenoh Publication Throughput Test")
        .arg(
            Arg::from_usage(
                "-p, --payload=[bytes] 'Payload size(bytes) for publication.'",
            )
            .default_value("1024"),
        )
        .arg(
            Arg::from_usage(
                "-n, --messages=[messages] 'Number of messages in each throughput measurements.'",
            )
            .default_value("1000000"),
        )
        .arg(
            Arg::from_usage(
                "-o, --loops=[loop] 'Number of loops for calculating average throughput.'",
            )
            .default_value("1"),
        )
        .arg(
            Arg::from_usage(
            "-e, --connect=[ENDPOINT]...  'Endpoints to connect to.'",
            )
        )
        .arg(
            Arg::from_usage(
                "-l, --listen=[ENDPOINT]...   'Endpoints to listen on.'",
            )
        )
        .get_matches();
    
    let mut config = Config::default();
    if let Some(values) = args.values_of("connect") {
        config
            .connect
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    if let Some(values) = args.values_of("listen") {
        config
            .listen
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    let payload: usize = args.value_of("payload").unwrap().parse().unwrap();
    let messages: usize = args.value_of("messages").unwrap().parse().unwrap();
    let loops: usize = args.value_of("loops").unwrap().parse().unwrap();
    
    (config, payload, messages, loops)
        
}