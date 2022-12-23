# Lottery finder

<div align="center">

  <h1><code>Lottery finder</code></h1>

  <strong>Find and check `Lotería de Navidad` numbers</strong>

  [![ci](https://github.com/jiep/lottery/actions/workflows/ci.yml/badge.svg)](https://github.com/jiep/lottery/actions/workflows/ci.yml)
  [![dependency status](https://deps.rs/repo/github/jiep/lottery/status.svg)](https://deps.rs/repo/github/jiep/lottery)

  <sub>Built with 🦀</sub>
</div>


## Binaries

Download the latest version from [Releases](https://github.com/jiep/lottery/releases).

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
./target/release/lottery # for release version
./target/debug/lottery # for debug version
```

## 🚴 Usage

```
A simple `Lotería de Navidad` finder and checker

Usage: lottery <COMMAND>

Commands:
  check  Check if a lottery number is prizewinning
  find   Find where a lottery number is located
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Check

```
Check if a lottery number is prizewinning

Usage: lottery check [OPTIONS]

Options:
  -n, --numbers <NUMBERS>...  
  -u, --url <URL>             Set url to download prizes from [default: https://www.loteriasyapuestas.es/servicios/premioDecimoWeb?idsorteo=1186309102]
  -h, --help                  Print help information
  -V, --version               Print version information
```

### Find

```
Find where a lottery number is located

Usage: lottery find [OPTIONS] --number <NUMBER>

Options:
  -n, --number <NUMBER>  Number to find
  -u, --url <URL>        Set url to download locations from [default: https://www.loteriasyapuestas.es/new-geo-web/JsonGenerationServlet/exportPois.txt?drawId=1186309102&number=]
  -j, --json             Return as json
  -h, --help             Print help information
  -V, --version          Print version information
```

### Examples

#### Check prizes for numbers 05490 12345 45544, and 88898 

```
lottery check --numbers 05490 12345 45544 88898

05490 => 400000 euros 🎉
12345 => 0 euros 😭
45544 => 0 euros 😭
88898 => 0 euros 😭

Total: 40000000 euros
```

#### Find locations for number 12345

```
lottery find --number 12345

Locations for 12345

Name: [Not set]
Address: ESTACIO, 3 MERCADO EXT. 1
City: LA LLAGOSTA
Province: BARCELONA
Phone: 935600094
Series: 1, 5, 6

Name: [Not set]
Address: DIAGONAL FIVALLER, 12
City: CARDEDEU
Province: BARCELONA
Phone: 938713319
Series: 3

Name: [Not set]
Address: SANTA ROSA, 7
City: CÓRDOBA
Province: CORDOBA
Phone: 957276608
Series: 4

Name: [Not set]
Address: ALAMEDA CRISTINA, 5
City: JEREZ DE LA FRONTERA
Province: CADIZ
Phone: 956343356
Series: 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180

Name: [Not set]
Address: CISNEROS, 9 -BAJO
City: MÁLAGA
Province: MALAGA
Phone: 952213622
Series: 2
```

#### Find locations for 12345 and show as JSON

```
lottery --number 12345 --json

{
  "number": 12345,
  "locations": [
    {
      "name": "",
      "address": "ESTACIO, 3 MERCADO EXT. 1",
      "city": "LA LLAGOSTA",
      "province": "BARCELONA",
      "phone": "935600094",
      "series": [
        1,
        5,
        6
      ]
    },
    {
      "name": "",
      "address": "DIAGONAL FIVALLER, 12",
      "city": "CARDEDEU",
      "province": "BARCELONA",
      "phone": "938713319",
      "series": [
        3
      ]
    },
    {
      "name": "",
      "address": "SANTA ROSA, 7",
      "city": "CÓRDOBA",
      "province": "CORDOBA",
      "phone": "957276608",
      "series": [
        4
      ]
    },
    {
      "name": "",
      "address": "ALAMEDA CRISTINA, 5",
      "city": "JEREZ DE LA FRONTERA",
      "province": "CADIZ",
      "phone": "956343356",
      "series": [
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        72,
        73,
        74,
        75,
        76,
        77,
        78,
        79,
        80,
        81,
        82,
        83,
        84,
        85,
        86,
        87,
        88,
        89,
        90,
        91,
        92,
        93,
        94,
        95,
        96,
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120,
        121,
        122,
        123,
        124,
        125,
        126,
        127,
        128,
        129,
        130,
        131,
        132,
        133,
        134,
        135,
        136,
        137,
        138,
        139,
        140,
        141,
        142,
        143,
        144,
        145,
        146,
        147,
        148,
        149,
        150,
        151,
        152,
        153,
        154,
        155,
        156,
        157,
        158,
        159,
        160,
        161,
        162,
        163,
        164,
        165,
        166,
        167,
        168,
        169,
        170,
        171,
        172,
        173,
        174,
        175,
        176,
        177,
        178,
        179,
        180
      ]
    },
    {
      "name": "",
      "address": "CISNEROS, 9 -BAJO",
      "city": "MÁLAGA",
      "province": "MALAGA",
      "phone": "952213622",
      "series": [
        2
      ]
    }
  ]
}
```

## License
This project is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.