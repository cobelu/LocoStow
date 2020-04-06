// arithmetic.rs
// Offers addition/subtraction abilities for a Hash

// The trie structure used is found here:
// https://docs.rs/radix_trie/0.1.6/radix_trie/struct.Trie.html

use super::hash;
use hash::Hash;

use patricia_tree::PatriciaMap;

pub fn new() -> PatriciaMap<Hash> {
    return PatriciaMap::new();
}

pub fn insert(tree: PatriciaMap<Hash>, hash: Hash) -> Option<Hash> {
    return tree.insert(hash.hash, hash);
}

pub fn get(tree: PatriciaMap<Hash>, hash: Hash) -> Option<&Hash> {
    return tree.get(hash.hash, Some(&hash));
}
