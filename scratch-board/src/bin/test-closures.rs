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

#[cfg(test)]
mod tests {
    #[test]
    fn f_test_closure_t1() {
        let m = 1.0;
        let n = 2.0;

        let line = |x| m * x + n;

        assert_eq!(line(0.0), 2.0);
        assert_eq!(line(1.0), 3.0);
    }

    #[test]
    fn f_test_closure_t2() {
        let mut m = 1.0;
        let mut n = 2.0;

        let line = move |x| m * x + n;

        n += 1.0;
        assert_eq!(line(0.0), 2.0); // note that m, n as initially captured from environment
        assert_eq!(m, 1.0);
        assert_eq!(n, 3.0);

        m += 1.0;
        assert_eq!(line(1.0), 3.0); // note that m, n as initially captured from environment
        assert_eq!(m, 2.0);
        assert_eq!(n, 3.0);
    }

    fn move_add(a: i32) -> impl Fn(i32) -> i32 {
        move |x| a + x
    }

    #[test]
    fn f_test_closure_t3() {
        let v = 2;
        let f = move_add(v);

        assert_eq!(f(1), 3);
    }

    #[test]
    fn f_test_closure_t4() {
        let mut v = vec![1; 4];
        assert_eq!(v.iter().filter(|&&n| n > 0).count(), 4);

        v.push(1);
        assert_eq!(v.iter().filter(|&&n| n > 0).count(), 5);

        v.push(1);
        assert_eq!(v.iter().filter(|&&n| n > 0).count(), 6);
    }
}
