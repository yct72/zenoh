use clap::{App, Arg};
use zenoh::config::Config;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::*;



fn main() {
    // initiate logging
    env_logger::init();
  
    let (config, payload, number, loops) = parse_args();

    // data
    let mut tempv = vec![0_u8; payload];
    for tv in tempv.iter_mut() {
        *tv = rand::random::<u8>();
    }
    let data: Value = tempv.into();

    // open session
    let session = zenoh::open(config).res().unwrap();

    // declare publisher
    let publisher = session
        .declare_publisher("test/thr")
        .congestion_control(CongestionControl::Block)
        .res().unwrap();

    let mut start_time = std::time::Instant::now();
    let mut avg_thr: f64 = 0.0;
    for l in 0..loops {
        start_time = std::time::Instant::now();
        for x in 0..number {
            publisher.put(data.clone()).res().unwrap();
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        let thr = number as f64 / elapsed;

        println!("[Loop {}]", l);
        println!("- Payload: {} bytes", payload);
        println!("- Throughput: {} msg/s", thr);
        println!("------------------------------------------");

        avg_thr += thr / loops as f64;
    }
    println!("");
    println!("*****************************************************************");
    println!("Zenoh Publication Throughput Test");
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