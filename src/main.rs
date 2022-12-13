
// use orderbook::OrderBook;

// mod orderbook;
// mod xmlparser;
// mod order;
use std::fs;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("hello there!");
    // Open and reads file to buffer
    let file_name = "test.txt";
    let mut file = File::open(file_name).expect("Unable to open file");
    let metadata = fs::metadata(&file_name).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    // Compresses buffer
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&buffer);
    let compressed_bytes = e.finish().expect("unable to compress bytes");

    // Decompresses buffer
    let mut d = ZlibDecoder::new(&*compressed_bytes);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    d.read(&mut buffer).expect("unable to read back");
    println!("File contents: {}", s);
}
