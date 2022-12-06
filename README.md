# rs-test

<div align="center">

  <h1><code>rs-test</code></h1>

  <strong>Implementation of `rs-test`</strong>

  [![ci](https://github.com/jiep/rs-test/actions/workflows/ci.yml/badge.svg)](https://github.com/jiep/rs-test/actions/workflows/ci.yml)
  [![dependency status](https://deps.rs/repo/github/jiep/rs-test/status.svg)](https://deps.rs/repo/github/jiep/rs-test)

  <sub>Built with 🦀</sub>
</div>


## Supported algorithms

<details>
  <summary>Click to expand supported hashes </summary>
  
    * SHA256
    * SHA512
    
</details>

## Binaries

Download the latest version from [Releases](https://github.com/jiep/rs-test/releases).

## Build from source

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Check source code

```
cargo check
``` 

3. Compile binary

```
cargo build
``` 

4. Run tests

```
cargo test
```

> Note: for release target, add --release

5. Run binary

```
cargo run
# or
./target/release/rs-test # for release version
./target/debug/rs-test # for debug version
```

## 🚴 Usage

```
Usage: rs-test --message <MESSAGE> --hash-algorithm <HASH_ALGORITHM>

Options:
      --message <MESSAGE>                
      --hash-algorithm <HASH_ALGORITHM>  
  -h, --help                             Print help information
  -V, --version                          Print version information
```

### Example

```
rs-test --message "hello world!" --hash-algorithm "sha256" --emojify
Message:    "hello world!"
Algorithm:  SHA256
Result:     7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9
Emojified:  👗🔟🐍7⃣
```

## License
This project is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.