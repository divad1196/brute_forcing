use rayon::{prelude::*, iter::plumbing::UnindexedConsumer};
use anyhow::{anyhow, Result};
use std::mem;

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

/*
pub struct Base {
    inner: Vec<usize>,
    base: usize,
}
impl Base {
    fn new(n: usize, base: usize) -> Option<Base> {
        match n_to_vec(n, base) {
            Some(vec) => Some(
                Base{
                    inner: vec,
                    base: base
                }
            ),
            None => None,
        }
    }
}
*/

pub struct BruteForce {
    current: Vec<usize>,
    size: usize,
}

impl BruteForce {
    pub fn new(size: usize) -> BruteForce {
        BruteForce {
            current: Vec::new(),
            size,
        }
    }
    pub fn chunks(mut self, count: usize, size: usize) -> Result<Vec<BruteForceChunk>> {
        if count < 2 {
            return Err(anyhow!("Count must be greater than 2"));
        }
        if size < 2 {
            return Err(anyhow!("Size must be greater than 2"));
        }
        let size = size - 1;
        let to_add = n_to_vec(size, self.size).unwrap();
        let mut vec = Vec::new();
        vec.reserve(count);
        for _ in 0..count {
            vec.push(
                BruteForceChunk {
                    bf: BruteForce {
                        current: self.next().unwrap(),
                        size: self.size
                    },
                    size,
                    count,
                    index: 0
                }
            );
            add_vec(&mut self.current, &to_add, self.size);
        }
        Ok(vec)
    }
    pub fn with_charset(&self, charset: &str) -> String {
        convert_vec(&self.current, &charset)
    }
}
impl Iterator for BruteForce {
    type Item = Vec<usize>;

     fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        loop {
            if self.current.len() == i {
                self.current.push(0);
                return Some(self.current.clone());
            }
            let x = self.current[i]
                .checked_add(1)
                .map(|x| x % self.size)
                .unwrap_or(0);
            self.current[i] = x;
            if x != 0 {
                break;
            }
            i += 1;
        }
        Some(self.current.clone())
    }

    // Documentation: override nth instead of "skip"
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let to_add = n_to_vec(n, self.size).unwrap();
        add_vec(&mut self.current, &to_add, self.size);
        Some(self.current.clone())
    }
    /* Experimental
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
    }*/
}

pub struct BruteForceChunk {
    bf: BruteForce,
    size: usize,
    count: usize, // Number of chunks
    index: usize,
}

impl BruteForceChunk {
    pub fn with_charset(&self, charset: &str) -> String {
        self.bf.with_charset(charset)
    }
}

impl Clone for BruteForce {
    fn clone(&self) -> BruteForce {
        BruteForce { current: self.current.clone(), size: self.size }
    }
}
impl Clone for BruteForceChunk {
    fn clone(&self) -> BruteForceChunk {
        BruteForceChunk {
            bf: self.bf.clone(),
            size: self.size,
            count: self.count,
            index: 0
        }
    }
}

impl Iterator for BruteForceChunk {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            self.index += 1;
        } else {
            self.index = 0;
            // We need to increment `count` time the value
            let to_add = n_to_vec(self.size, self.size).unwrap();
            for _ in 0..self.count {
                add_vec(&mut self.bf.current, &to_add, self.bf.size);
            }

        }
        self.bf.next()
    }

    // Documentation: override nth instead of "skip"
    /*fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let to_add = n_to_vec(self.size, self.size).unwrap();
        add_vec(&mut self.current, &to_add, self.size);
        Some(self.current.clone())
    }*/
}


/*
pub trait ParallelIterator: Sized + Send {
    type Item: Send;
    fn drive_unindexed<C>(self, consumer: C) -> C::Result where
        C: UnindexedConsumer<Self::Item> {

        }
}*/
/*
impl IntoParallelIterator for BruteForce{
    type Iter: ParallelIterator<Item = Self::Item>;
    type Item: Send;

    fn into_par_iter(self) -> Self::Iter {

    }
}*/


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
                size: charset.len(),
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
            .map(|res| convert_vec(&res, &self.charset))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.brute_force.nth(n).map(|res| convert_vec(&res, &self.charset))
    }
}
