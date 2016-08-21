# Sanchez

Rust playground project.

Currently finds `*.mp3` files in a given directory and parses them ID3 tags. The
parsing can be done multithreaded, defining the number of threads with the `-j`
option.

Besides the "scanner" functionality there is also a "watcher", which watches the
exact same given directory for file changes, resulting in the same ID3 extraction
process.

## Build & Run

Since some dependencies require the unstable `Plugin` feature, we need to have
a nightly version of Rust installed. The easiest way is using
[`rustup`](https://www.rustup.rs/)

```
rustup install nightly
rustup default nightly

cargo build
time cargo run -- ~/Music -j4 -vv -w
```

see: https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust


### Links

#### Environment

- [`rustup` announcement](https://blog.rust-lang.org/2016/05/13/rustup.html)

#### Threads

- [Threadpool](https://frewsxcv.github.io/rust-threadpool/threadpool/index.html)

#### Strings

- [Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)
- [string vs. str](http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)

#### Crypto

- http://siciarz.net/24-days-of-rust-rust-crypto/

#### Framework

- [Web Frameworks](https://github.com/flosse/rust-web-framework-comparison)

