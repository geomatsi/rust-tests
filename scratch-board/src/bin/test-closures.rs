use std::thread;
use std::time::Duration;

fn sim_long_calc(intensity: u32) -> u32 {
    println!("slow calculation...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn sim_generate_workout(intensity: u32, rand: u32) {
    let calc_res = sim_long_calc(intensity);

    if intensity >= 10 {
        println!("Run {} km", calc_res);
    } else {
        if rand == 3 {
            println!("Time to recover, take a break today!");
        } else {
            println!("Recovery run {} km", calc_res);
        }
    }
}

fn main() {
    println!(">>>>> Closure examples");

    sim_generate_workout(10, 7);
}
