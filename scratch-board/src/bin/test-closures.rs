use std::thread;
use std::time::Duration;


fn sim_generate_workout(intensity: u32, rand: u32) {

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
    println!(">>>>> Closure examples");

    sim_generate_workout(10, 7);
}
