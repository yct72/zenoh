use zenoh::{config::Config, key_expr};
// use zenoh::prelude::r#async::*;
use clap::{App, Arg};
// use zenoh::prelude::sync::*;
use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    /* 
        Send and Sync: https://doc.rust-lang.org/nomicon/send-and-sync.html
        A type is Send if it is safe to send it to another thread.
        A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).
    */
    
    let (config, key_expr) = (Config::default(), "test/thr");

    println!("Opening Zenoh session...");
    let session = zenoh::open(config).res().await.unwrap();
    
    println!("Allocating Shared Memory Buffer...");
    let subscriber = session.declare_subscriber(key_expr).res().await.unwrap();

    loop {
        subscriber.recv_async().await.unwrap();
        println!("Received!");
    }

    
} 

