use clap::{App, Arg};
use zenoh::config::Config;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::*;



fn main() {
    // initiate logging
    env_logger::init();
  
    let config = Config::default();
    let (payload, number, loops) = parse_args();

    // data
    let mut tempv = vec![0_u8; payload];
    for tv in tempv.iter_mut() {
        *tv = rand::random::<u8>() % 10;
    }
    let data: Value = tempv.into();

    // open session
    let session = zenoh::open(config).res().unwrap();

    // declare publisher
    let publisher = session
        .declare_publisher("test/thr").res().unwrap();

    let mut start_time = std::time::Instant::now();
    let mut avg_thr: f64 = 0.0;
    for l in 0..loops {
        start_time = std::time::Instant::now();
        for x in 0..number {
            publisher.put(data.clone()).res().unwrap();
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        let thr = number as f64 / elapsed;
        println!("payload: {}", payload);
        println!("{} msg/s", thr);
        println!("---------------------");

        avg_thr = (avg_thr + thr) / (l as f64 + 1.0);
    }
    println!("-----------------------------------------------");
    println!("payload: {}", payload);
    println!("{} msg/s", avg_thr);
    println!("---------------------");
    
   
}


fn parse_args() -> (usize, usize, usize) {
    
    let args = App::new("Zenoh Publication Throughput Test")
        .arg(
            Arg::from_usage(
                "-p, --payload=[payload] 'Payload size for publication.'",
            )
            .default_value("1024"),
        )
        .arg(
            Arg::from_usage(
                "-n, --messages=[messages] 'Number of messages in each throughput measurements.'",
            )
            .default_value("100000"),
        )
        .arg(
            Arg::from_usage(
                "-l, --loops=[loop] 'Number of loops for calculating average throughput.'",
            )
            .default_value("10000"),
        )
        .get_matches();
    
    let payload: usize = args.value_of("payload").unwrap().parse().unwrap();
    let messages: usize = args.value_of("messages").unwrap().parse().unwrap();
    let loops: usize = args.value_of("loops").unwrap().parse().unwrap();

    (payload, messages, loops)
        
}