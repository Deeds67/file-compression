
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
use std::str;

fn main() {
    println!("hello there!");
    // Open and reads file to buffer
    let file_name = String::from("test.txt");
    let buffer = read_file(&file_name);

    // Compresses buffer
    let compressed_bytes = compress_buffer(&buffer);

    // Decompresses buffer
    let decompressed_bytes = decompress_buffer(compressed_bytes);

    let file_contents = str::from_utf8(&decompressed_bytes).expect("Expected file contents to be string");

    println!("{}", file_contents);
}

fn read_file(file_name: &String) -> Vec<u8> {
    let mut file = File::open(file_name).expect("Unable to open file");
    let metadata = fs::metadata(file_name).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");
    return buffer;
}

fn compress_buffer(buffer: &Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(buffer).expect("Could not compress buffer");
    return e.finish().expect("unable to compress bytes");
}

fn decompress_buffer(buffer: Vec<u8>) -> Vec<u8> {
    let mut d = ZlibDecoder::new(&*buffer);
    // let mut s = String::new();
    // d.read_to_string(&mut s).unwrap();
    let mut buffer = vec![0; buffer.len() as usize];
    d.read(&mut buffer).expect("unable to read back");
    return buffer
}
