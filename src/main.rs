mod arithmetic;
mod hash;
mod trie;
mod file;

use hash::*;
use std::process;
use file::readIn;

fn main() {
    let swi: Point = Point {
        lat: 33.6472022,
        lon: -96.5987648,
        time: 1585512888000000000,
    };

    let pvd: Point = Point {
        lat: 41.8269387,
        lon: -71.4017563,
        time: 1586182020000000000,
    };

    let swi_hash: Hash = encode(swi, 20);
    println!("{}", swi_hash);

    let pvd_hash: Hash = encode(pvd, 20);
    println!("{}", pvd_hash);

    let swi_output: Output = decode(swi_hash);
    println!("{}", swi_output);

    let pvd_output: Output = decode(pvd_hash);
    println!("{}", pvd_output);

    let point_add: Point = swi + pvd;
    println!("{}", point_add);

    if let Err(err) = readIn() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
