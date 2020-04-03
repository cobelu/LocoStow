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
    let point: Point = output.point;
    let error: Error = output.error;
    println!(
        "({}, {}, {}) w/ error ({}, {}, {})",
        point.lat, point.lon, point.time, error.lat_err, error.lon_err, error.time_err
    );
}
