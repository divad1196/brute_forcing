use std::{cmp::Ordering, collections::HashSet};

use brute_forcing::BruteForce;
use rayon::prelude::*;
use anyhow::{Result};

fn str_cmp(s1: &String, s2: &String) -> Ordering {
    if s1.len() == s2.len() {
        return s1.cmp(s2);
    }
    s1.len().cmp(&s2.len())
}

fn test_parallel() -> Result<Vec<String>> {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
    const CHUNK_SIZE: usize = 500;
    let charset_size: usize = CHARSET.chars().count();
    let res = BruteForce::new(charset_size).chunk_vec(1, CHUNK_SIZE);
    if let Err(e) = res {
        eprintln!("{}", e);
        return Err(e);
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
    for s in res.iter() {
        println!("{}", s);
    }
    Ok(res)
}


fn compute_parallel(charset: &str, chunk_size: usize, chunk_count: usize) -> Result<Vec<String>> {
    let charset_size: usize = charset.chars().count();
    let res = BruteForce::new(charset_size).chunk_vec(chunk_count, chunk_size);
    if let Err(e) = res {
        eprintln!("{}", e);
        return Err(e);
    }
    let mut chunks = res.unwrap();
    let res: Vec<String> = chunks
        .par_iter_mut()
        .map(|c| {
            c
                .take_while(|b| b.get().len() < 3)
                .map(|v| v.with_charset(charset))
        })
        .flatten_iter()
        .collect();
    Ok(res)
}
fn compute_serial(charset: &str) -> Result<Vec<String>> {
    let charset_size: usize = charset.chars().count();
    let brute = BruteForce::new(charset_size);
    let res: Vec<String> = brute
        .into_iter()
        .take_while(|b| b.get().len() < 3)
        .map(|c| {
            c.with_charset(charset)
        })
        .collect();
    Ok(res)
}

#[test]
fn test_chunk() -> Result<()> {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut par_result = HashSet::new();
    let mut ser_result = HashSet::new();
    par_result.extend(compute_parallel(CHARSET, 15, 5)?.iter().cloned());
    ser_result.extend(compute_serial(CHARSET)?.iter().cloned());
    let diff: Vec<&String> = par_result.symmetric_difference(&ser_result).collect();
    assert!(diff.is_empty(), "Difference between 2 modes:\nMissing: {:?}\nExtra: {:?}", ser_result.difference(&par_result), par_result.difference(&ser_result));
    Ok(())
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
    // cargo tests hides output on success: https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests
    // we can use `cargo test  -- --nocapture`
    // --nocapture is an option for the test binary, not for cargo
    println!("{:?}", res);
    assert!(res != None)
}

/*
#[test]
fn main() {
    test_parallel();
    test_find_value();
}
*/
