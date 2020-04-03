mod hash;

// Avoid multiple compilations
// https://docs.rs/regex/latest/regex/
// #[macro_use] extern crate lazy_static;

use hash::*;

fn main() {
    let point: Point = Point {
        lat: 33.6633807,
        lon: -96.6410105,
        time: 1585512888000000000,
    };

    let hash: String = encode_hash(point, 20);
    println!("{}", hash);

    let output: Output = decode(hash);
    println!("{}", output);
}
