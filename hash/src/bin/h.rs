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
