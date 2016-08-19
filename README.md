# Sanchez

Rust playground project.

Currently finds `*.mp3` files in a given directory and parses them ID3 tags. The
parsing can be done multithreaded, defining the number of threads with the `-j`
option.

Besides the "scanner" functionality there is also a "watcher", which watches the
exact same given directory for file changes, resulting in the same ID3 extraction
process.

## Run
```
cargo build
time cargo run -- ~/Music -j4 -vvv -w)
```

### Links

#### Threads

- [Threadpool](https://frewsxcv.github.io/rust-threadpool/threadpool/index.html)

#### Strings

- [Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)
- [string vs. str](http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html)