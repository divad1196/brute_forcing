mod lib;
use lib::StringBruteForce;

fn main() {
    for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz")
        .skip(1_000_000)
        .take(1_000_000)
    {
        println!("{}", text);
    }
}
