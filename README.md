# Brute Force

An easy way to generate strings for brute-forcing

```rust
for text in StringBruteForce::new("abcdefghijklmnopqrstuvwxyz").take(1_000_000) {
    println!("{}", text);
}
```

