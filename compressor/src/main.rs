use flate2::{write::GzEncoder, Compression};
//flate ka crate use krte for compression to gzip
use std::env::args;
//to take the environment ke arguments
use std::fs::File;
//to open close and work with the files
use std::io::copy;
//taaki aaram se copy kr paye files contents to the output file
use std::io::BufReader;
//to be able to read the file contents from the file
use std::time::{Instant};
//to find the time that has elapsed till now

fn main(){
    if args().len() !=3{
        eprintln!("Usage: 'source' 'target'");
        return;
    }
    //agar teen arguments nahi diye mtlb , hmne command , input output , nahi dia to raise a print in the error stream

    let mut input  = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //opens a bufReader stream
     let output = File::create(args().nth(2).unwrap()).unwrap();
     //encoder stream - mei output aur compression level bhejenge
     let mut encoder = GzEncoder::new(output, Compression::default());
     //ye yaha pr ek encoding stream open kr deta hai , and ab usko use , finish ye sb krna pdega
     let start = Instant::now();
     copy(&mut input , &mut encoder).unwrap();
     //copy - copies the entire content from a reader to a writer
     let output = encoder.finish().unwrap();
     println!(
        "Source len: {}", input.get_ref().metadata().unwrap().len()
        //get_ref - gets the reference to the underlying reader, which here is BufReader
     );
     println!("Output length : {:?} ", output.metadata().unwrap().len());
    println!("Elapsed Time : {:?}", start.elapsed());
}