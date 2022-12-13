
// use orderbook::OrderBook;

// mod orderbook;
// mod xmlparser;
// mod order;
use std::fs;

fn main() {
    println!("hello there!");
    let data = fs::read_to_string("test.txt").expect("Unable to read file");
    println!("{}", data);
}
