# Sanchez

Rust playground project.

Currently finds `*.mp3` files in a given directory and parses them ID3 tags. The
parsing can be done multithreaded, defining the number of threads with the `-j`
option.

Besides the "scanner" functionality there is also a "watcher", which watches the
exact same given directory for file changes, resulting in the same ID3 extraction
process.

## Build & Run

### Prerequisites
Since some dependencies require the unstable `Plugin` feature, we need to have
a nightly version of Rust installed. The easiest way is using
[`rustup`](https://www.rustup.rs/)

```
rustup install nightly
rustup default nightly
```

see: https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust

### Develop
Once rust is setup, we can compile and start

```
cargo build && time RUST_BACKTRACE=1 target/debug/sanchez ~/Music/ -vv
```

Note we don't call `cargo run`, see "Issues" below


## Issues

When running `cargo build && time RUST_BACKTRACE=1 cargo run -- /tmp/  -vvv`
we receive an error that goes back to `cargo`
```
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/sanchez /tmp/ -vvv -w`
dyld: Symbol not found: __cg_jpeg_resync_to_restart
  Referenced from: /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
  Expected in: /opt/local/lib/libJPEG.dylib
 in /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
error: Process didn't exit successfully: `target/debug/sanchez /tmp/ -vvv -w` (signal: 5, SIGTRAP: trace/breakpoint trap)
```


Some values in `arguments.rs` are `warn`ed to be unused, though they are
used
```
warning: value assigned to `verbosity` is never read, #[warn(unused_assignments)] on by default
  --> src/arguments.rs:72:9
   |
72 |     let mut verbosity = 0;
   |         ^^^^^^^^^^^^^

warning: struct field is never used: `thread_number`, #[warn(dead_code)] on by default
  --> src/scan.rs:22:5
   |
22 |     thread_number: usize,
   |     ^^^^^^^^^^^^^^^^^^^^

```

## Links

### Environment

- [`rustup` announcement](https://blog.rust-lang.org/2016/05/13/rustup.html)

### Threads

- [Threadpool](https://frewsxcv.github.io/rust-threadpool/threadpool/index.html)

### Strings

- [Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)
- [string vs. str](http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)

### Crypto

- http://siciarz.net/24-days-of-rust-rust-crypto/

### Framework

- [Web Frameworks](https://github.com/flosse/rust-web-framework-comparison)

