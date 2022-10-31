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

    fn next(&mut self) ->Option<Self::Item> {
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
}

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
        self.brute_force.next().map(|res| convert_vec(&res, &self.charset))
    }
}

fn main() {
    for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz").take(1_000_000) {
        println!("{}", text);
    }
}
