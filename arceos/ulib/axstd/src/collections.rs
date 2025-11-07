#![cfg(feature = "alloc")]
use axhal::misc::random;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use alloc::boxed::Box;
pub use alloc::collections::VecDeque;

// extern crate alloc;
// pub use alloc::collections;
const TABLE_SIZE: usize = 50101;
pub struct HashMap<K, V> {
    pairs: Box<[Option<(K, V)>]>,
    step: u128, // a random probe step
}

impl<K, V> HashMap<K, V> {
    pub fn hash(key: &K) -> u64 {
        unsafe {
            let input = core::slice::from_raw_parts(key as *const K as *const u8, core::mem::size_of::<K>());
            const_xxh3(input) % (TABLE_SIZE as u64)
        }
    }

    pub fn new() -> Self {
        HashMap {  
            pairs: Box::new([const {None}; TABLE_SIZE]),
            step: random() % TABLE_SIZE as u128,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut ind = Self::hash(&key);

        while let Some(_value) = &self.pairs[ind as usize] {
            ind = (ind + self.step as u64) % TABLE_SIZE as u64;
        }

        self.pairs[ind as usize] = Some((key, value));
    }

    pub fn iter(&self) -> HashMapIter<K, V> {
        HashMapIter { pairs: &self.pairs, cur_pos: 0 }
    }
}

pub struct HashMapIterator<K, V> {
    pairs: Box<[Option<(K, V)>]>,
    cur_pos: usize,
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = HashMapIterator<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator {
            pairs: self.pairs,
            cur_pos: 0,
        }
    }
}

impl<K, V> Iterator for HashMapIterator<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_pos < TABLE_SIZE {
            if let Some(pair) = self.pairs[self.cur_pos].take() {
                self.cur_pos += 1;
                return Some(pair);
            }

            self.cur_pos += 1;
        }

        return None;
    }
}

pub struct HashMapIter<'a, K: 'a, V: 'a> {
    pairs: &'a [Option<(K, V)>],
    cur_pos: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_pos < TABLE_SIZE {
            if let Some(pair) = &self.pairs[self.cur_pos] {
                self.cur_pos += 1;
                return Some((&pair.0, &pair.1));
            }
            self.cur_pos += 1;
        }
        return None;
    }
}
