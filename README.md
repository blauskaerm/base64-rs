# Base64-rs

A base64 encoder/decoder written in Rust

```
Base64 encode/decode data and print to standard output

USAGE:
    base64 [FLAGS] [OPTIONS] [FILE]

FLAGS:
    -d, --decode            Decode data
    -h, --help              Prints help information
    -i, --ignore-garbage    When decoding, ignore non-alphabet characters
    -V, --version           Prints version information

OPTIONS:
    -w, --wrap <COLS>    Wrap encoded lines after COLS characters (default 76)

ARGS:
    <FILE>    File to encode/decode, or - to read from stdin

```

## Inspiration

* https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64#Decode
* https://stackoverflow.com/questions/11559203/decode-table-construction-for-base64

