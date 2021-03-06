// file.rs
// Offers file reading capabilities

use super::hash;
use hash::*;
use std::collections::BTreeSet;
use patricia_tree::PatriciaMap;
use std::error::Error;
use std::fs::File;
use heapsize::heap_size_of;
use std::mem::{size_of, size_of_val};
use fossil_delta::delta;
use std::time::Instant;
use std::io;

pub fn read_in() -> Result<(), Box<dyn Error>> {
    // 2 reads allows size for vector creation - avoids resizing during inserts. Unsure if tradeoff is worth it.

    let file2 =
    File::open("/Users/cobelu/Documents/School/CSCI2270/brown-cs227-tsbs-help/data/fake-data")?;

    let mut rdrcnt = csv::Reader::from_reader(file2);

    let mut cnt = 0;
    let mut bytes = 0;
    for result in rdrcnt.records() {
        cnt += 1;
    }

    //not necessary, can read and send to hashes right away - experiment with vec population
    //went with assigned size vector - easy to manipulate/populate, minimal values need to be retained through copies
    //obvious tradeoff - heap v stack
    let now = Instant::now();
    let file =
    File::open("/Users/cobelu/Documents/School/CSCI2270/brown-cs227-tsbs-help/data/fake-data")?;
    let mut rdr = csv::Reader::from_reader(file);
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
    let point_read = now.elapsed().as_nanos();
    println!("{} {}","Point Read:",point_read);
    let mut hash_vec: Vec<String> = Vec::new();
    hash_vec.reserve_exact(cnt);
    let delta_list =list.clone();

    for point in list {
        let hash: Hash = encode(point, 24);
        bytes += hash.hash.len();
        hash_vec.push(hash.hash);
    }
    let point_hash_read = now.elapsed().as_nanos();
    println!("{} {}","Point and Hash Read:",point_hash_read);
    println!("{} {}","Hash Overhead",point_hash_read - point_read);
    println!("{} {}","Avg Time to encode Hash Point",(point_hash_read - point_read)/cnt as u128);

    println!("{}", &hash_vec[0].len());
    println!("{}", bytes / cnt);

    //Delta Encode GT Hash
    let mut delta_vec: Vec<Vec<u8>> = Vec::new();
    delta_vec.reserve_exact(cnt);
    for x in 0..hash_vec.len() - 1 {
        let curr = &hash_vec[x];
        let next = &hash_vec[x + 1];
        let delta = delta(curr, next);
        delta_vec.push(delta);
    }
    let delta_string_read = now.elapsed().as_nanos();
    println!("{} {}","Delta Encode w GeoHash:",delta_string_read);
    println!("{} {}","Delta String Overhead",delta_string_read - point_hash_read);

    let mut delta_vec_float: Vec<Point> = Vec::new();
    delta_vec_float.reserve_exact(cnt);
    for x in 0..delta_list.len()-1{
        let curr = delta_list[x].clone();
        let next = delta_list[x + 1].clone();
        let delta_pt = next-curr;
        delta_vec_float.push(delta_pt);
    }
    let delta_float_read = now.elapsed().as_nanos();
    println!("{} {}","Delta Encode w Points:",delta_float_read);
    println!("{} {}","Delta Float Overhead",delta_float_read - delta_string_read);

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
/*
pub fn read_to_btree_pt() -> BTreeSet<Point> {
    let mut b_tree = BTreeSet::new();
     let file =
         File::open(io::stdin()).unwrap();
     let mut rdr = csv::Reader::from_reader(file);
     let mut bytes = 0;
     for result in rdr.records().into_iter() {
         let record = result.unwrap();
         let latitude: f64 = record[1].parse().unwrap();
         let longitude: f64 = record[2].parse().unwrap();
         let t: i64 = record[0].parse().unwrap();
         let pt: Point = Point {
             lat: latitude,
             lon: longitude,
             time: t,
         };
         b_tree.insert(pt);
    }
     return b_tree
 }
*/
pub fn read_to_btree_hash() -> BTreeSet<String> {
    
    let mut b_tree = BTreeSet::new();
    let file =
    File::open("/Users/cobelu/Documents/School/CSCI2270/brown-cs227-tsbs-help/data/fake-data").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut hash_vec: Vec<String> = Vec::new();
    //let mut bytes = 0;
    for result in rdr.records().into_iter() {
        let record = result.unwrap();
        let latitude: f64 = record[1].parse().unwrap();
        let longitude: f64 = record[2].parse().unwrap();
        let t: i64 = record[0].parse().unwrap();
        let pt: Point = Point {
            lat: latitude,
            lon: longitude,
            time: t,
        };
        let hash: Hash = encode(pt, 24);
        hash_vec.push(hash.hash);
        
    }
    let now = Instant::now();
    for hash_str in hash_vec{
        b_tree.insert(hash_str);
    }
    println!("{} {}","BTree Insert:", now.elapsed().as_millis());
    return b_tree
}

pub fn read_to_ptree() -> PatriciaMap<String>{
    let now = Instant::now();
    let mut p_tree = PatriciaMap::new();
    let file =
    File::open("/Users/cobelu/Documents/School/CSCI2270/brown-cs227-tsbs-help/data/fake-data").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
   // let mut bytes = 0;
    let mut hash_vec: Vec<String> = Vec::new();
    for result in rdr.records().into_iter() {
        let record = result.unwrap();
        let latitude: f64 = record[1].parse().unwrap();
        let longitude: f64 = record[2].parse().unwrap();
        let t: i64 = record[0].parse().unwrap();
        let pt: Point = Point {
            lat: latitude,
            lon: longitude,
            time: t,
        };
        let hash: String = encode(pt, 24).hash;
        
        hash_vec.push(hash);
    }
    let now = Instant::now();
    for hash_str in hash_vec{
        let value = hash_str.clone();
        p_tree.insert(hash_str,value);
    }
    println!("{} {}","PTree Insert:", now.elapsed().as_millis());

    return p_tree
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_read_in() {
        let result = read_in();
    }
    /*
        fn test_size_of_btree_pt() {
            println!("{}", mem::size_of_val(&read_to_btree_pt()))
        }
    */
    #[test]
    fn test_size_of_btree_hash() {
        let btree_test = read_to_btree_hash();
        println!("{}", mem::size_of_val(&btree_test));
        let now = Instant::now();
        // let get_val = btree_test.get("PQkpOx8rKTkfKx0p");
        //println!("{:?}",get_val);
        println!("{} {} {}","BTree Get:", now.elapsed().as_nanos(), "nanoseconds");
    }
    
    #[test]
    fn test_size_of__ptree() {
        let ptree_test = read_to_ptree();
        println!("{}", mem::size_of_val(&ptree_test));
        let now = Instant::now();
        // let get_Val = ptree_test.get("PQkpOx8rKTkfKx0p");
        //println!("{:?}",get_Val);
        println!("{} {} {}","PTree Get:", now.elapsed().as_nanos(), "nanoseconds");
    }

}
