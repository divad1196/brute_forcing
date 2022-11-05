use anyhow::{anyhow, Result};
// use rayon::prelude::*;

fn n_to_vec(n: usize, base: usize) -> Option<Vec<usize>> {
    if base == 0 {
        return None;
    }
    let mut n = n;
    let mut vec = Vec::new();
    while n > 0 {
        let current = n % base;
        n = (n - current) / base;
        vec.push(current);
    }
    Some(vec)
}

fn add_vec(dest: &mut Vec<usize>, other: &Vec<usize>, base: usize) {
    if base == 0 {
        return;
    }
    let mut ret = 0usize;
    let mut other = other.clone();

    while dest.len() < other.len() {
        dest.push(0);
    }
    while other.len() < dest.len() {
        other.push(0);
    }

    let mut index = 0usize;
    while index < other.len() {
        let tmp = dest[index] + other[index] + ret;
        let current = tmp % base;
        ret = (tmp - current) / base;
        dest[index] = current;
        index += 1;
    }
    if ret != 0 {
        dest.push(ret);
    }
}

pub struct BruteForceValue(Vec<usize>);

impl BruteForceValue {
    pub fn get(&self) -> &Vec<usize> {
        &self.0
    }

    pub fn with_charset(&self, charset: &str) -> String {
        convert_vec(&self.0, &charset)
    }
}

#[derive(Debug)]
pub struct BruteForce {
    current: Vec<usize>,
    base: usize,
}

impl BruteForce {
    pub fn new(base: usize) -> BruteForce {
        BruteForce {
            current: Vec::new(),
            base,
        }
    }

    pub fn chunk_vec(mut self, count: usize, size: usize) -> Result<Vec<BruteForceChunk>> {
        if count < 1 {
            return Err(anyhow!("Count must be non null"));
        }
        if size < 2 {
            return Err(anyhow!("Size must be greater than 2"));
        }
        let to_add = n_to_vec(size, self.base).unwrap();
        let mut vec = Vec::new();
        vec.reserve(count);
        for _ in 0..count {
            vec.push(BruteForceChunk {
                bf: BruteForce {
                    current: self.current.clone(),
                    base: self.base,
                },
                size,
                count,
                index: 0,
            });
            add_vec(&mut self.current, &to_add, self.base);
        }
        Ok(vec)
    }

    pub fn chunks(self, count: usize, size: usize) -> Result<BruteForcePool> {
        Ok(BruteForcePool(self.chunk_vec(count, size)?))
    }

    pub fn with_charset(&self, charset: &str) -> String {
        convert_vec(&self.current, &charset)
    }
}

impl Iterator for BruteForce {
    type Item = BruteForceValue;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        loop {
            if self.current.len() == i {
                self.current.push(0);
                return Some(BruteForceValue(self.current.clone()));
            }
            let x = self.current[i]
                .checked_add(1)
                .map(|x| x % self.base)
                .unwrap_or(0);
            self.current[i] = x;
            if x != 0 {
                break;
            }
            i += 1;
        }
        Some(BruteForceValue(self.current.clone()))
    }

    // Documentation: override nth instead of "skip"
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let to_add = n_to_vec(n, self.base).unwrap();
        add_vec(&mut self.current, &to_add, self.base);
        Some(BruteForceValue(self.current.clone()))
    }
}

#[derive(Debug)]
pub struct BruteForceChunk {
    bf: BruteForce,
    size: usize,
    count: usize, // Number of chunks
    index: usize,
}

pub struct BruteForcePool(Vec<BruteForceChunk>);

impl BruteForceChunk {
    pub fn with_charset(&self, charset: &str) -> String {
        self.bf.with_charset(charset)
    }
}

impl Clone for BruteForce {
    fn clone(&self) -> BruteForce {
        BruteForce {
            current: self.current.clone(),
            base: self.base,
        }
    }
}
impl Clone for BruteForceChunk {
    fn clone(&self) -> BruteForceChunk {
        BruteForceChunk {
            bf: self.bf.clone(),
            size: self.size,
            count: self.count,
            index: 0,
        }
    }
}

impl Iterator for BruteForceChunk {
    type Item = BruteForceValue;

    fn next(&mut self) -> Option<Self::Item> {
        // let tmp = self.bf.current.clone();
        if self.index == self.size - 1 {
            self.index = 0;
            // We need to increment `count` time the value
            let to_add = n_to_vec(self.size, self.bf.base).unwrap();
            for _ in 0..(self.count - 1) {
                add_vec(&mut self.bf.current, &to_add, self.bf.base);
            }
        }
        self.index += 1;
        self.bf.next()
    }
}

pub struct StringBruteForce {
    brute_force: BruteForce,
    charset: String,
}

pub fn convert_vec(vec: &Vec<usize>, charset: &str) -> String {
    String::from_iter(vec.iter().filter_map(|i| charset.chars().nth(*i as usize)))
}

impl StringBruteForce {
    pub fn new(charset: &str) -> StringBruteForce {
        StringBruteForce {
            brute_force: BruteForce {
                current: Vec::new(),
                base: charset.len(),
            },
            charset: charset.to_string(),
        }
    }

    pub fn convert(&self) -> String {
        convert_vec(&self.brute_force.current, &self.charset)
    }
}
impl Iterator for StringBruteForce {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.brute_force
            .next()
            .map(|res| convert_vec(&res.0, &self.charset))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.brute_force
            .nth(n)
            .map(|res| convert_vec(&res.0, &self.charset))
    }
}
