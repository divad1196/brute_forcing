# Brute Force

An easy way to generate strings for brute-forcing

```rust
for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz").take(1_000_000) {
    println!("{}", text);
}
```



### Parallel Computing using [Rayon](https://docs.rs/rayon/latest/rayon/)

Currently, the library does not directly implement any trait from Rayon, but it provides you a way to use it:

```rust
const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
const CHUNK_SIZE: usize = 500;
let charset_size: usize = CHARSET.chars().count();

// This provides you a Vec<BruteForceChunk>. BruteForceChunk are iterable that won't overlap over each other
let chunks = BruteForce::new(charset_size).chunk_vec(CHUNK_NUMBER, CHUNK_SIZE).unwrap();
```

Now you can use your chunks, e.g. to find a word 

```rust
const TO_FIND: &str = "hello";
// use `find_map_first` to stop all parallel iterables once you find what you want.
let res = chunks.par_iter_mut().find_map_first(|c| {
        c
    		// convert your chunk to a string using a charset
            .map(|v| v.with_charset(charset))
    		// Add any kind of limit if you want, otherwise the program may run forever until it finds something
            	//.take_while(|s| s.len() < 6)
    		// Stop when you find what you want.
            .find(|s| {
                // you could do something more interesting like comparing strings hashes
                s.eq(TO_FIND)  
            });
});
```

