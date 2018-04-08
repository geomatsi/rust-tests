extern crate rand;

use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn sim_generate_workout(intensity: u32) {
    let rand = rand::thread_rng().gen_range(1, 8);

    let opt_res = |num: u32| -> u32 {
        println!("slow calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity >= 10 {
        println!("Run {} km", opt_res(intensity));
    } else {
        if rand == 3 {
            println!("Time to recover, take a break today!");
        } else {
            println!("Recovery run {} km", opt_res(intensity));
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Input workout intensity:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let intensity: u32 =
        input.trim().parse().expect("failed to parse input");

    sim_generate_workout(intensity);
}
