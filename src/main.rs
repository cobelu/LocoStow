mod hash;
mod arithmetic;
mod trie;

use hash::*;
use std::error::Error;
use std::fs::File;
use std::{process, mem};
use std::mem::{size_of, size_of_val};
use heapsize::heap_size_of;
use fossil_delta::delta;


fn readIn() -> Result<(), Box<dyn Error>> {
    // 2 reads allows size for vector creation - avoids resizing during inserts. Unsure if tradeoff is worth it.
    let file = File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let file2 = File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rdrcnt = csv::Reader::from_reader(file2);

    let mut cnt = 0;
    let mut bytes = 0;
    for result in rdrcnt.records() {
        cnt +=1;
    }

    //not necessary, can read and send to hashes right away - experiment with vec population
    //went with assigned size vector - easy to manipulate/populate, minimal values need to be retained through copies
    //obvious tradeoff - heap v stack
    let mut list: Vec<Point> = Vec::new();
    list.reserve_exact(cnt);
    for result in rdr.records() {
        let record = result?;
        let latitude: f64 = record[1].parse()?;
        let longitude: f64 = record[2].parse()?;
        let t: i64 = record[0].parse()?;

        let pt: Point = Point {
            lat:latitude,
            lon:longitude,
            time: t,
        };
        list.push(pt);
        //println!("{:?}", record);
        //cnt +=1;
    }

    let mut hashVec: Vec<String> = Vec::new();
    hashVec.reserve_exact(cnt);
    for point in list{
        let hash: Hash = encode(point,24);
        bytes += hash.hash.len();
        hashVec.push(hash.hash);

    }
    //Should be the same
    println!("{}",&hashVec[0].len());
    println!("{}",bytes/cnt);

    //experiment w/ deltas, need to retain 1 hash to decode
    let mut deltaVec: Vec<Vec<u8>> = Vec::new();
    deltaVec.reserve_exact(cnt);
    for x in 0..hashVec.len()-1 {
        let curr = &hashVec[x];
        let next = &hashVec[x+1];
        let delta = delta(curr, next);
        deltaVec.push(delta);
    }
    Ok(())
}
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