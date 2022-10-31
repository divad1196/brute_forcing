use rayon::{prelude::*, iter::plumbing::UnindexedConsumer};
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
    charset_size: usize,
}

impl BruteForce {
    fn new(charset_size: usize) -> BruteForce {
        BruteForce {
            current: Vec::new(),
            charset_size: charset_size,
        }
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
                .map(|x| x % self.charset_size)
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
        let to_add = n_to_vec(n, self.charset_size).unwrap();
        add_vec(&mut self.current, &to_add, self.charset_size);
        Some(self.current.clone())
    }
    /* Experimental
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
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

fn convert_vec(vec: &Vec<usize>, charset: &str) -> String {
    String::from_iter(vec.iter().filter_map(|i| charset.chars().nth(*i as usize)))
}

impl StringBruteForce {
    pub fn new(charset: &str) -> StringBruteForce {
        StringBruteForce {
            brute_force: BruteForce {
                current: Vec::new(),
                charset_size: charset.len(),
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
