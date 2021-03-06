mod arithmetic;
mod file;
mod hash;
mod trie;

use file::{read_in, read_to_btree_hash, read_to_ptree};
use hash::*;
use std::process;

fn main() {
        let swi: Point = Point {
            lat: 33.6472022,
            lon: -96.5987648,
            time: 1585512888000000000,
        };

    //     let pvd: Point = Point {
    //         lat: 41.8269387,
    //         lon: -71.4017563,
    //         time: 1586182020000000000,
    //     };

        let swi_hash: Hash = encode(swi, 20);
        println!("{}", swi_hash);

        let size1 = std::mem::size_of_val(&swi);
        println!("{}", size1);
        let size2 = std::mem::size_of_val(&swi_hash.hash);
        println!("{}", size2);

    //     let pvd_hash: Hash = encode(pvd, 20);
    //     println!("{}", pvd_hash);

    //     let swi_output: Output = decode(swi_hash);
    //     println!("{}", swi_output);

    //     let pvd_output: Output = decode(pvd_hash);
    //     println!("{}", pvd_output);

    //     let point_add: Point = swi + pvd;
    //     println!("{}", point_add);

    //  let result = read_in();
    //  if result.is_ok() {
    //      println!("{}", "Success!");
    //  }

    // let result = read_to_btree_hash();

   // let result = read_to_ptree();
}
