use futures::executor::block_on;

async fn fib(n: u32) -> u32 {
    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 0..n {
        n2 = n1 + n2;
        n1 = n2 - n1;
    }

    n2
}

fn main() {
    for i in 1..10 {
        let f = fib(i);
        let val = block_on(f);
        println!("fib({}) = {}", i, val);
    }
}
