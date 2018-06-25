extern crate rand;

use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn sim_generate_workout(intensity: u32) {
    let rand = rand::thread_rng().gen_range(1, 8);

    let mut opt_res = Cacher::new(|num: u32| -> u32 {
        println!("slow calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity >= 10 {
        println!("Run {} km", opt_res.value(intensity));
    } else {
        if rand == 3 {
            println!("Time to recover, take a break today!");
        } else {
            println!("Recovery run {} km", opt_res.value(intensity));
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Input workout intensity:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let intensity: u32 = input.trim().parse().expect("failed to parse input");

    sim_generate_workout(intensity);
}
