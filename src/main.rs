mod lib;
use brute_forcing::{BruteForce, convert_vec};
use lib::StringBruteForce;
use rayon::prelude::*;

fn main() {
    /*for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz")
        .skip(1_000_000)
        .take(1_000_000)
    {
        println!("{}", text);
    }*/
    let CHUNK_SIZE = 500usize;
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let charset_size = charset.chars().count();
    let res = BruteForce::new(charset_size).chunks(5, CHUNK_SIZE);
    if let Err(e) = res {
        eprintln!("{}", e);
        return;
    }
    let mut chunks = res.unwrap();
    for (i, c) in chunks.iter_mut().enumerate() {
        let tmp = c.with_charset(charset);
        // let value = convert_vec(&tmp, charset);
        println!("Chunk {}: {:?}", i, tmp);
    }
    /*let chunk =  chunks[0].clone();
    for c in chunk.take(CHUNK_SIZE + 2) {
        let value = convert_vec(&c, charset);
        println!("{:?}", value);
    }*/

    chunks.par_iter_mut().enumerate().for_each(|(i, c)| {
        c.for_each(|value| {
            println!("Chunk {}: {}", i, convert_vec(&value, charset))
        })
    })
}
