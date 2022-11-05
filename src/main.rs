mod lib;
use std::cmp::Ordering;

use brute_forcing::BruteForce;
use rayon::prelude::*;

fn str_cmp(s1: &String, s2: &String) -> Ordering {
    if s1.len() == s2.len() {
        return s1.cmp(s2);
    }
    s1.len().cmp(&s2.len())
}

fn test_parallel() {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
    const CHUNK_SIZE: usize = 500;
    let charset_size: usize = CHARSET.chars().count();
    let res = BruteForce::new(charset_size).chunk_vec(1, CHUNK_SIZE);
    if let Err(e) = res {
        eprintln!("{}", e);
        return;
    }
    let mut chunks = res.unwrap();
    println!("chunks: {:?}", chunks);
    let mut res: Vec<String> = chunks
        .par_iter_mut()
        .map(|c| {
            c.map(|v| v.with_charset(CHARSET))
                .take_while(|s| s.len() < 3)
        })
        .flatten_iter()
        .collect();
    res.sort_by(str_cmp);
    for s in res {
        println!("{}", s);
    }
}

#[allow(unused)]
fn test_find_value() {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let charset_size = charset.chars().count();
    const CHUNK_SIZE: usize = 500usize;
    let res = BruteForce::new(charset_size).chunk_vec(5, CHUNK_SIZE);
    if let Err(e) = res {
        eprintln!("{}", e);
        return;
    }
    let mut chunks = res.unwrap();
    const TO_FIND: &str = "hello";
    let res = chunks.par_iter_mut().enumerate().find_map_first(|(i, c)| {
        let res = c
            .map(|v| v.with_charset(charset))
            .take_while(|s| s.len() < 6)
            .find(|s| {
                s.eq(TO_FIND)
            });
        match res {
            Some(s) => Some((i, s)),
            None => None,
        }
    });
    println!("Found: {:?}", res);
}

/*
#[allow(unused)]
fn test1() {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz")
        .skip(1_000_000)
        .take(1_000_000)
    {
        println!("{}", text);
    }
}*/

fn main() {
    test_parallel();
    test_find_value();
}
