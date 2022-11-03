use zenoh::buffers::SharedMemoryManager;
use std::time::Duration; 
use zenoh::config::Config;
use async_std::task::sleep;
use zenoh::prelude::r#async::*;
use clap::{App, Arg};

const N: usize = 10;
const K: u32 = 3;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /* 
        Send and Sync: https://doc.rust-lang.org/nomicon/send-and-sync.html
        A type is Send if it is safe to send it to another thread.
        A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).
    */
    
    // let config = Config::default();
    // let path = String::from("my_practice_key/exp");
    // let value = "my practice publish shm!";
    let (config, path, value) = parse_args();

    println!("Opening Zenoh session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Creating Shared Memory Manager...");
    let id = session.zid();
    let mut shm = SharedMemoryManager::make(id.to_string(), N * 1024).unwrap();
    
    println!("Allocating Shared Memory Buffer...");
    let publisher = session.declare_publisher(&path).res().await.unwrap();

    // TODO
    for idx in 0..(K * N as u32) {
        let mut sbuf = match shm.alloc(1024) {
            Ok(buf) => buf,
            Err(_) => {
                sleep(Duration::from_millis(100)).await;
                println!(
                    "After failing allocation the GC collected: {} bytes -- retrying",
                    shm.garbage_collect()
                );
                println!(
                    "Trying to de-fragment memory... De-fragmented {} bytes",
                    shm.defragment()
                );
                shm.alloc(1024).unwrap()
            }
        };
    

        // reserve at the beginning of the buffer to include the iteration index of the write
        // the same format as zn_pub
        let prefix = format!("[{:4}] ", idx);
        let prefix_len = prefix.as_bytes().len();

        // retrieve a mutable slice from the SharedMemoryBuf.
        let slice = unsafe { sbuf.as_mut_slice() }; // sbuf
        let slice_len = prefix_len + value.as_bytes().len(); // idx + publish value
        slice[0..prefix_len].copy_from_slice(prefix.as_bytes());
        slice[prefix_len..slice_len].copy_from_slice(value.as_bytes());

        // write data
        println!(
            "Put SHM Data('{}': '{}')", 
            path,
            String::from_utf8_lossy(&slice[0..slice_len])
        );
        publisher.put(sbuf.clone()).res().await?;
        if idx % K == 0 {
            let freed = shm.garbage_collect();
            println!("The Garbage Collector freed {} bytes", freed);
            let defrag = shm.defragment();
            println!("De-fragmented {} bytes", defrag);
        }

        drop(sbuf);
        
    }

    let _freed = shm.garbage_collect();
    
    Ok(())

}

fn parse_args() -> (Config, String, String) {
    let args = App::new("zenoh shared-memory pub example")
        .arg(
            Arg::from_usage("-m, --mode=[MODE] 'The zenoh session mode (peer by default).")
                .possible_values(&["peer", "client"]),
        )
        .arg(Arg::from_usage(
            "-e, --connect=[ENDPOINT]...  'Endpoints to connect to.'",
        ))
        .arg(Arg::from_usage(
            "-l, --listen=[ENDPOINT]...   'Endpoints to listen on.'",
        ))
        .arg(
            Arg::from_usage("-p, --path=[PATH]        'The key expression to publish onto.'")
                .default_value("demo/example/zenoh-rs-pub"),
        )
        .arg(
            Arg::from_usage("-v, --value=[VALUE]      'The value of to publish.'")
                .default_value("Pub from SharedMemory Rust!"),
        )
        .arg(Arg::from_usage(
            "-c, --config=[FILE]      'A configuration file.'",
        ))
        .arg(Arg::from_usage(
            "--no-multicast-scouting 'Disable the multicast-based scouting mechanism.'",
        ))
        .get_matches();

    let mut config = if let Some(conf_file) = args.value_of("config") {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };
    if let Some(Ok(mode)) = args.value_of("mode").map(|mode| mode.parse()) {
        config.set_mode(Some(mode)).unwrap();
    }
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
    if args.is_present("no-multicast-scouting") {
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
    }

    let path = args.value_of("path").unwrap();
    let value = args.value_of("value").unwrap();

    (config, path.to_string(), value.to_string())
}
