use zenoh::prelude::sync::*;
fn main() {
    let data: Value = (0usize..10)
        .map(|i| (i % 10) as u8)
        .collect::<Vec<u8>>()
        .into();
    println!("data: {:?}", data);
    println!("(0usize..10): {:?}", (0usize..10));
    let a = (0usize..10).map(|i| (i % 10) as u8);
    println!("map = {:?}", a);
    println!("collect = {:?}", a.collect::<Vec<u8>>());
}
