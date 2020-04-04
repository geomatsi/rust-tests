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
