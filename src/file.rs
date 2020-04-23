// file.rs
// Offers file reading capabilities

use super::hash;
use hash::*;
use trie::*;

use std::collections::BTreeSet;
use patricia_tree::PatriciaMap;
use std::error::Error;
use std::fs::File;
use heapsize::heap_size_of;
use std::mem::{size_of, size_of_val};
use fossil_delta::delta;

pub fn readIn() -> Result<(), Box<dyn Error>> {
    // 2 reads allows size for vector creation - avoids resizing during inserts. Unsure if tradeoff is worth it.
    let file =
        File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let file2 =
        File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rdrcnt = csv::Reader::from_reader(file2);

    let mut cnt = 0;
    let mut bytes = 0;
    for result in rdrcnt.records() {
        cnt += 1;
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
            lat: latitude,
            lon: longitude,
            time: t,
        };
        list.push(pt);
        //println!("{:?}", record);
        //cnt +=1;
    }

    let mut hashVec: Vec<String> = Vec::new();
    hashVec.reserve_exact(cnt);
    for point in list {
        let hash: Hash = encode(point, 24);
        bytes += hash.hash.len();
        hashVec.push(hash.hash);
    }
    //Should be the same
    println!("{}", &hashVec[0].len());
    println!("{}", bytes / cnt);

    //experiment w/ deltas, need to retain 1 hash to decode
    let mut deltaVec: Vec<Vec<u8>> = Vec::new();
    deltaVec.reserve_exact(cnt);
    for x in 0..hashVec.len() - 1 {
        let curr = &hashVec[x];
        let next = &hashVec[x + 1];
        let delta = delta(curr, next);
        deltaVec.push(delta);
    }

    let swi_pt: Point = Point {
        lat: 33.6472022,
        lon: -96.5987648,
        time: 1585512888000000000,
    };

    let swi_hash: String = "KDI0NiYFFiAXDzoKES0H".to_string();

    println!("Size of the Hash: {}", size_of_val(&swi_hash));
    println!("Size of the Point: {}", size_of_val(&swi_pt));

    Ok(())
}

pub fn read_to_btree_hash() -> &BTreeSet {
    let mut b_tree = BTreeSet::new();
    let file =
        File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut bytes = 0;
    for result in rdr.records() {
        let record = result?;
        let latitude: f64 = record[1].parse()?;
        let longitude: f64 = record[2].parse()?;
        let t: i64 = record[0].parse()?;
        let pt: Point = Point {
            lat: latitude,
            lon: longitude,
            time: t,
        };
        b_tree.insert(pt)
    }
    return &b_tree
}

pub fun read_to_btree_hash() -> &BTreeSet {
    let mut b_tree = BTreeSet::new();
    let file =
        File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut bytes = 0;
    for result in rdr.records() {
        let record = result?;
        let latitude: f64 = record[1].parse()?;
        let longitude: f64 = record[2].parse()?;
        let t: i64 = record[0].parse()?;
        let pt: Point = Point {
            lat: latitude,
            lon: longitude,
            time: t,
        };
        let hash: String = encode(pt, 24)
        b_tree.insert(hash.hash)
    }
    return &b_tree
}

pub fun read_to_ptree() -> &PatriciaMap {
    let mut p_tree = PatriciaMap::new();
    let file =
        File::open("C:/Users/chris/Documents/GitHub/brown-cs227-tsbs-help/data/fake-data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut bytes = 0;
    for result in rdr.records() {
        let record = result?;
        let latitude: f64 = record[1].parse()?;
        let longitude: f64 = record[2].parse()?;
        let t: i64 = record[0].parse()?;
        let pt: Point = Point {
            lat: latitude,
            lon: longitude,
            time: t,
        };
        let hash: String = encode(pt, 24).hash
        p_tree.insert(hash)
    }
    return &p_tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_in() {
        let result = readIn();
    }

    fn test_size_of_btree_pt() {
        println!("{}", mem::size_of_val(read_to_btree_pt()))
    }

    fn test_size_of_btree_hash() {
        println!("{}", mem::size_of_val(read_to_btree_hash()))
    }

    fn test_size_of__ptree() {
        println!("{}", mem::size_of_val(read_to_ptree()))
    }

}
