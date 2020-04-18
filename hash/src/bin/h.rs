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

// Example #8
//
// Two strings are isomorphic if the characters in s can be replaced to get t.
// Given two strings s and t, determine if they are isomorphic.
//
// All occurrences of a character must be replaced with another character while
// preserving the order of characters. No two characters may map to the same
// character but a character may map to itself.
//
// Note: you may assume both s and t have the same length

fn string_to_set(s: String) -> HashSet<Vec<usize>> {
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    let mut set: HashSet<Vec<usize>> = HashSet::new();

    for (n, c) in s.chars().enumerate() {
        map.entry(c).or_insert_with(|| vec![]).push(n);
    }

    for v in map.values() {
        set.insert(v.clone());
    }

    set
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let s1: HashSet<Vec<usize>> = string_to_set(s);
    let s2: HashSet<Vec<usize>> = string_to_set(t);

    s1 == s2
}

#[test]
fn test_isomorphic() {
    assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
    assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    assert_eq!(
        is_isomorphic("paper".to_string(), "title".to_string()),
        true
    );
}

// Example #9
//
// Consider two lists of favorite restaurants represented by strings
//
// Find out common interest with the least list index sum. If there
// is a choice tie between answers, output all of them with no order
// requirement. You could assume there always exists an answer.
//
// Notes:
// - the length of both lists will be in the range of [1, 1000]
// - the length of strings in both lists will be in the range of [1, 30]
// - the index is starting from 0 to the list length minus 1
// - no duplicates in both lists

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut m: HashMap<String, usize> = HashMap::new();

    let mut res: Vec<&str> = vec![];
    let mut min: usize = 10000;

    for (n, v) in list2.iter().enumerate() {
        m.insert(v.to_string(), n);
    }

    for (n, k) in list1.iter().enumerate() {
        if let Some(m) = m.get(k) {
            match n + m {
                s if s < min => {
                    res.clear();
                    res.push(k);
                    min = s;
                }
                s if s == min => {
                    res.push(k);
                }
                _ => {}
            };
        }
    }

    res.iter()
        .map(|s| (*s).to_string())
        .collect::<Vec<String>>()
}

#[test]
fn test_restraunts() {
    let a = ["Shogun", "Tapioca Express", "Burger King", "KFC"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let b = [
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();
    let c = ["Shogun"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    assert_eq!(find_restaurant(a, b), c);

    let a = ["Shogun", "Tapioca Express", "Burger King", "KFC"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let b = ["KFC", "Shogun", "Burger King"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let c = ["Shogun"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    assert_eq!(find_restaurant(a, b), c);
}

// Example #10
//
// Given a string, find the first non-repeating character in it and return it's index.
// If it doesn't exist, return -1
//
// Note
// You may assume the string contain only lowercase letters.

pub fn first_uniq_char(s: String) -> i32 {
    let mut m: HashMap<char, usize> = HashMap::new();
    let mut n: Vec<usize> = vec![0; s.len()];

    for (p, c) in s.chars().enumerate() {
        match m.get(&c) {
            Some(q) => {
                n[*q] = 1;
                n[p] = 1;
            }
            None => {
                m.insert(c, p);
            }
        }
    }

    for (i, v) in n.iter().enumerate() {
        if *v == 0 {
            return i as i32;
        }
    }

    -1
}

#[test]
fn test_uniq_char() {
    assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(first_uniq_char("abab".to_string()), -1);
    assert_eq!(first_uniq_char("abba".to_string()), -1);
}

// Example #11
//
// Given two arrays, write a function to compute their intersection.
//
// Each element in the result should appear as many times as it shows in both arrays.
// The result can be in any order.

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut res: Vec<i32> = vec![];

    for k in nums1.iter() {
        map.entry(*k).and_modify(|e| *e += 1).or_insert(1);
    }

    for k in nums2.iter() {
        map.entry(*k).and_modify(|e| {
            if *e > 0 {
                *e -= 1;
                res.push(*k);
            }
        });
    }

    res
}

#[test]
fn test_intersect() {
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![9, 4]);
}

// Example #12
//
// Given an array of integers and an integer k, find out whether there are two distinct indices
// i and j in the array such that nums[i] = nums[j] and the absolute difference between
// i and j is at most k.

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut m: HashMap<i32, usize> = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        if let Some(j) = m.get_mut(v) {
            if (i - *j) as i32 <= k {
                return true;
            }
            *j = i;
        } else {
            m.insert(*v, i);
        }
    }

    false
}

#[test]
fn test_nearby_duplicate() {
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}

// Example #13
//
// Given an array of strings, group anagrams together.

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    let mut res: Vec<Vec<String>> = vec![];

    for s in strs {
        let mut ch: Vec<char> = s.chars().collect::<Vec<char>>();
        ch.sort();
        map.entry(ch).or_insert_with(Vec::new).push(s);
    }

    for v in map.values() {
        res.push(v.to_vec());
    }

    res
}

#[test]
fn test_group_anagrams() {
    let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let mut angs: Vec<Vec<String>> = vec![
        vec!["eat", "tea", "ate"]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
        vec!["tan", "nat"]
            .into_iter()
            .map(|x| x.to_string())
            .collect(),
        vec!["bat"].into_iter().map(|x| x.to_string()).collect(),
    ];

    let mut strs = group_anagrams(strs);

    strs.sort();
    angs.sort();

    assert_eq!(strs, angs);
}
