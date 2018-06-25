use std::clone;

//

fn main() {
    println!("run tests: cargo test --bin test-iterators");
}

//

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    model: String,
}

impl clone::Clone for Shoe {
    fn clone(&self) -> Shoe {
        Shoe {
            size: self.size,
            model: self.model.clone(),
        }
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(m: u32) -> Counter {
        Counter { count: 0, max: m }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

impl clone::Clone for Counter {
    fn clone(&self) -> Counter {
        Counter {
            count: self.count,
            max: self.max,
        }
    }
}

fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .cloned()
        .filter(|s| s.size <= shoe_size)
        .collect()
}

#[test]
fn f_test_iter_shoes() {
    let shoes = vec![
        Shoe {
            size: 10,
            model: String::from("Asics"),
        },
        Shoe {
            size: 13,
            model: String::from("Mizuno"),
        },
        Shoe {
            size: 10,
            model: String::from("Hoka"),
        },
        Shoe {
            size: 11,
            model: String::from("Saucony"),
        },
    ];

    let shoes_list = shoes.iter();
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    // ok to use 'shoes' after .iter() over refs
    let shoes_list = shoes_in_my_size(&shoes, 11);
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    // ok to use 'shoes' after 'shoes_in_my_size': no ownership changes
    let shoes_list = shoes.iter();
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    // ok to use 'shoes' after .iter() over refs
    println!("Shoe list {:?}", shoes);
}

#[test]
fn f_test_iter_counter() {
    let c0 = Counter::new(5);

    // iter cloned c0
    for s in c0.clone() {
        println!("counter values: {}", s);
    }

    // ok to use iterator c0 after using clone
    let c1 = c0.clone();
    assert_eq!(c1.collect::<Vec<u32>>(), vec![1, 2, 3, 4, 5]);

    // ok to use iterator c0 after using its clone
    let c2 = c0.clone().map(|x| x * x).filter(|x| (*x < 15));
    assert_eq!(c2.collect::<Vec<u32>>(), vec![1, 4, 9]);

    // ok to use c0 after ops with its clone
    for s in c0 {
        println!("counter: {}", s);
    }

    // Notes:
    //  - c0 has been moved to 'for': can't use it
    //  - move occurs because `c0` has type `Counter`,
    //    which does not implement the `Copy` trait
}

// function that takes ownership
fn bogus_var_move<T>(names: Vec<T>)
where
    T: std::fmt::Debug,
{
    for s in names {
        println!("counter: {:?}", s);
    }
}

// use iter on array of &str
#[test]
fn f_test_iter_ref() {
    let names = vec!["a", "b", "c"];

    let bytes = names
        .iter()
        .map(|n: &&str| n.len())
        .fold(0, |acc, len| acc + len);

    assert_eq!(bytes, 3);

    bogus_var_move(names);

    // 'names' has been moved: can't be used
}

#[test]
fn f_test_iter_val() {
    let mut names: Vec<String> = Vec::new();

    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");

    names.push(a);
    names.push(b);
    names.push(c);

    assert_eq!(names, vec!["a", "b", "c"]);

    let bytes = names
        .iter()
        .map(|n: &String| n.len())
        .fold(0, |acc, len| acc + len);

    assert_eq!(bytes, 3);

    bogus_var_move(names);

    // 'names' has been moved: can't be used
}

#[test]
fn f_test_loops() {
    let values = vec!["a", "b", "c"];

    // 'for' loop is syntactic sugar for into_iter
    // use ref in 'for' in other to use iterable data further
    for x in &values {
        println!("{}", x);
    }

    assert_eq!(values, vec!["a", "b", "c"]);

    for x in values {
        println!("{}", x);
    }

    // 'values' has been moved into 'for' loop: can't be used
}

#[test]
fn f_test_into_iter() {
    let v1 = vec![
        "hello".to_string(),
        "funny".to_string(),
        "world".to_string(),
    ];

    let v2: Vec<String> = v1.iter().cloned().filter(|s| s.contains('o')).collect();
    let v3: Vec<String> = v1
        .clone()
        .into_iter()
        .filter(|s| !s.contains('f'))
        .collect();
    let v4: Vec<String> = v1.into_iter().filter(|s| !s.contains('l')).collect();

    assert_eq!(v2, vec!["hello", "world"]);
    assert_eq!(v3, vec!["hello", "world"]);
    assert_eq!(v4, vec!["funny"]);
}
