use std::env::args;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read, Write, copy};
use std::time::Instant;

fn main() {
    if args().len() < 4 {
        eprintln!("Usage: <input> <compressed> <decompressed>");
        return;
    }

    let input_path = args().nth(1).unwrap();
    let compressed_path = args().nth(2).unwrap();
    let decompressed_path = args().nth(3).unwrap();

    let time = Instant::now();

    let input_file = File::open(&input_path).unwrap();
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(&compressed_path).unwrap();
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    copy(&mut reader, &mut encoder).unwrap();
    let compressed_file = encoder.finish().unwrap();

    println!("--- Compression Done ---");
    println!("Input len: {}", File::open(&input_path).unwrap().metadata().unwrap().len());
    println!("Compressed len: {}", compressed_file.metadata().unwrap().len());
    println!("Time elapsed: {:?}", time.elapsed());

    let time2 = Instant::now();

    let compressed_reader = File::open(&compressed_path).unwrap();
    let mut decoder = GzDecoder::new(compressed_reader);
    let mut decompressed_output = File::create(&decompressed_path).unwrap();

    copy(&mut decoder, &mut decompressed_output).unwrap();

    println!("\n--- Decompression Done ---");
    println!("Compressed len: {}", File::open(&compressed_path).unwrap().metadata().unwrap().len());
    println!("Decompressed len: {}", File::open(&decompressed_path).unwrap().metadata().unwrap().len());
    println!("Time elapsed: {:?}", time2.elapsed());

}
