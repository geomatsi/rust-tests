use std::clone;

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

fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .cloned()
        .filter(|s| s.size <= shoe_size)
        .collect()
}

fn main() {
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
    println!("ALL Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    let shoes_list = shoes_in_my_size(&shoes, 11);
    println!("My Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    let shoes_list = shoes.iter();
    println!("ALL Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }
}
