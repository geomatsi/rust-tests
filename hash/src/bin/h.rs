use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("run tests: cargo test");
}

// Example #1: design a HashSet without using any built-in hash table libraries
//
// To be specific, your design should include these functions:
//   - add(value)
//     -- insert a value into the HashSet
//   - contains(value)
//     -- return whether the value exists in the HashSet or not
//   - remove(value)
//     -- remove a value in the HashSet; if the value does not exist in the HashSet, do nothing
//
// Notes:
// All values will be in the range of [0, 1000000].
// The number of operations will be in the range of [1, 10000].
// Please do not use the built-in HashSet library.
//

struct MyHashSet {
    s: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        let s = vec![vec![]; 100];

        MyHashSet { s }
    }

    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.s[MyHashSet::h_func(key)].push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        if self.contains(key) {
            let idx = MyHashSet::h_func(key);

            for (n, v) in self.s[idx].iter().enumerate() {
                if v == &key {
                    self.s[idx].remove(n);
                    return;
                }
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.s[MyHashSet::h_func(key)].contains(&key)
    }

    fn h_func(v: i32) -> usize {
        if v < 0 {
            ((-v) % 100) as usize
        } else {
            (v % 100) as usize
        }
    }
}

#[test]
fn test_my_hash_set() {
    let mut hs = MyHashSet::new();

    assert_eq!(hs.contains(1), false);
    assert_eq!(hs.contains(2), false);
    assert_eq!(hs.contains(3), false);

    hs.add(1);
    hs.add(2);
    assert_eq!(hs.contains(1), true);
    assert_eq!(hs.contains(3), false);

    hs.add(2);
    assert_eq!(hs.contains(2), true);

    hs.remove(2);
    assert_eq!(hs.contains(2), false);
}

// Example #2: design a HashMap without using any built-in hash table libraries
//
// To be specific, your design should include these functions:
// - put(key, value)
//   -- insert a (key, value) pair into the HashMap; if the value already exists in the HashMap, update the value
// - get(key)
//   -- returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
// - remove(key)
//   -- remove the mapping for the value key if this map contains the mapping for the key
//
// Notes:
// - all keys and values will be in the range of [0, 1000000]
// - the number of operations will be in the range of [1, 10000]
// - please do not use the built-in HashMap library
//

struct MyHashMap {
    s: Vec<Vec<(i32, i32)>>,
}

#[allow(dead_code)]
impl MyHashMap {
    fn new() -> Self {
        let s = vec![vec![]; 100];

        MyHashMap { s }
    }

    fn put(&mut self, key: i32, value: i32) {
        let (kx, nx) = self.locate(key);

        match nx {
            Some(n) => self.s[kx][n].1 = value,
            None => self.s[kx].push((key, value)),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let (kx, nx) = self.locate(key);

        match nx {
            Some(n) => self.s[kx][n].1,
            None => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        let (kx, nx) = self.locate(key);

        if let Some(n) = nx {
            self.s[kx].remove(n);
        }
    }

    fn locate(&self, key: i32) -> (usize, Option<usize>) {
        let kx = MyHashMap::h_func(key);

        for (n, v) in self.s[kx].iter().enumerate() {
            if v.0 == key {
                return (kx, Some(n));
            }
        }

        (kx, None)
    }

    fn h_func(k: i32) -> usize {
        if k < 0 {
            ((-k) % 100) as usize
        } else {
            (k % 100) as usize
        }
    }
}

#[test]
fn test_my_hash_map() {
    let mut hm = MyHashMap::new();

    hm.put(1, 1);
    hm.put(2, 2);
    assert_eq!(hm.get(1), 1);
    assert_eq!(hm.get(2), 2);
    assert_eq!(hm.get(3), -1);
    assert_eq!(hm.get(100), -1);

    hm.put(2, 1);
    hm.put(1, 2);
    assert_eq!(hm.get(2), 1);
    assert_eq!(hm.get(1), 2);
    assert_eq!(hm.get(100), -1);

    hm.remove(2);
    assert_eq!(hm.get(1), 2);
    assert_eq!(hm.get(2), -1);
}

// Example #3: given an array of integers, find if the array contains any duplicates
//
// Your function should return true if any value appears at least twice in the array,
// and it should return false if every element is distinct.

#[allow(dead_code)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for v in nums.iter() {
        if set.contains(v) {
            return true;
        }

        set.insert(v);
    }

    false
}

#[test]
fn test_contains_duplicante() {
    assert_eq!(contains_duplicate(vec![]), false);
    assert_eq!(contains_duplicate(vec![1]), false);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}

// Example #4
//
// Given a non-empty array of integers, every element appears twice except for one. Find that single one.
// Your algorithm should have a linear runtime complexity.

#[allow(dead_code)]
fn single_number(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();

    for v in nums.iter() {
        if set.contains(v) {
            set.remove(v);
        } else {
            set.insert(v);
        }
    }

    // input data sanity check
    assert_eq!(set.len(), 1);

    *set.into_iter().collect::<Vec<&i32>>().pop().unwrap()
}

#[test]
fn test_single_number() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}

// Example #5
//
// Given two arrays, write a function to compute their intersection.
//
// Notes:
// - each element in the result must be unique
// - the result can be in any order

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let s1: HashSet<i32> = nums1.into_iter().collect();
    let s2: HashSet<i32> = nums2.into_iter().collect();

    s1.intersection(&s2).copied().collect::<Vec<i32>>()
}

#[test]
fn test_intersection() {
    assert_eq!(
        intersection(vec![1, 2, 2, 1], vec![2, 2])
            .into_iter()
            .collect::<HashSet<i32>>(),
        vec![2].into_iter().collect::<HashSet<i32>>()
    );
    assert_eq!(
        intersection(vec![9, 4, 9, 8, 4], vec![4, 9, 5])
            .into_iter()
            .collect::<HashSet<i32>>(),
        vec![4, 9].into_iter().collect::<HashSet<i32>>()
    );
}

// Example #6
//
// Write an algorithm to determine if a number is "happy".
//
// A happy number is a number defined by the following process: Starting with any positive integer,
// replace the number by the sum of the squares of its digits, and repeat the process until the
// number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy numbers.

fn digits(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut n = n;

    loop {
        if n == 0 {
            return v;
        }

        v.push(n - (n / 10) * 10);
        n /= 10;
    }
}

pub fn is_happy(n: i32) -> bool {
    let mut t: HashSet<i32> = HashSet::new();
    let mut n = n;

    loop {
        let digits = digits(n);
        n = digits.iter().copied().map(|x| x * x).sum();

        if n == 1 {
            return true;
        }

        if t.contains(&n) {
            return false;
        }

        t.insert(n);
    }
}

#[test]
fn test_happiness() {
    assert_eq!(is_happy(0), false);
    assert_eq!(is_happy(1), true);
    assert_eq!(is_happy(19), true);
}

// Example #7
//
// Given an array of integers, return indices of the two numbers
// such that they add up to a specific target.
//
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (n, v) in nums.iter().enumerate() {
        match map.get(v) {
            Some(m) => return vec![*m as i32, n as i32],
            None => {
                map.insert(target - *v, n);
            }
        }
    }

    vec![]
}

#[test]
fn test_two_sums() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(two_sum(vec![0, 2], 2), [0, 1]);
    assert_eq!(two_sum(vec![0, 1, 2, 3], 2), [0, 2]);
}
