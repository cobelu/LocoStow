// arithmetic.rs
// Offers addition/subtraction abilities for a Hash

// The trie structure used is found here:
// https://docs.rs/radix_trie/0.1.6/radix_trie/struct.Trie.html

use super::hash;
use hash::*;

use patricia_tree::PatriciaMap;

pub fn tree_new() -> PatriciaMap<Hash> {
    return PatriciaMap::new();
}
/*
pub fn insert(tree: PatriciaMap<Hash>, hash: Hash) -> Option<Hash> {
    return tree.insert(hash.hash, hash);
}

pub fn get(tree: PatriciaMap<Hash>, hash: Hash) -> Option<&Hash> {
    return tree.get(hash.hash, Some(&hash));
}
*/

//Introduces another layer of borrowing - tree consumed, lack of self param
pub fn tree_insert(mut tree: PatriciaMap<Hash>, hash: Hash) -> Option<Hash> {
    let key = hash.hash.clone();
    return tree.insert(key, hash);
}

//Introduces another layer of borrowing - tree consumed, lack of self param
pub fn tree_get(tree: &patricia_tree::map::PatriciaMap<hash::Hash>, hash: Hash) -> Option<&Hash> {
    let val = tree.get(hash.hash);
    return val;
}

pub fn range_query(start: Point, end: Point, ) {
    // Encode the start and stop (with max precision)
    let start_hash = encode(start, 30);
    let end_hash = encode(end, 30);
    // Get the shared prefix
}

#[cfg(test)]
mod tests {
    // Franco: now this super will see high_or_low_time because it's a submodule
    use super::*;

    #[test]
    fn create_test(){
        let tree = tree_new();
    }

    #[test]
    fn insert_test(){
        let mut tree = tree_new();
        let pvd: Point = Point {
            lat: 41.8269387,
            lon: -71.4017563,
            time: 1586182020000000000,
        };

        let pvd_hash: Hash = encode(pvd, 20);
        println!("{}", pvd_hash);
        tree_insert(tree,pvd_hash);
    }

    #[test]
    fn get_test(){
        let mut tree = PatriciaMap::new();
        let pvd: Point = Point {
            lat: 41.8269387,
            lon: -71.4017563,
            time: 1586182020000000000,
        };

        let pvd_hash: Hash = encode(pvd, 20);
        //println!("{}", pvd_hash);
        let key = pvd_hash.hash.clone();
        let value = key.clone();
        tree.insert(key,value);
        tree.insert("Ky07Lw0234", "Ky07Lw0234".parse().unwrap());

        let get_ref = tree.get(&pvd_hash.hash);

        //println!("{:?}", get_ref);
        assert_eq!(get_ref,Some(&pvd_hash.hash));

    }

}