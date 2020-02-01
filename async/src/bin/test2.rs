use futures::executor::block_on;
use futures::join;

async fn fib(n: u32) -> u32 {
    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 0..n {
        n2 += n1;
        n1 = n2 - n1;
    }

    n2
}

async fn async_join(
    t1: impl futures::future::Future<Output = u32>,
    t2: impl futures::future::Future<Output = u32>,
) -> (u32, u32) {
    join!(t1, t2)
}

fn main() {
    for i in 1..10 {
        let f = fib(i);
        let val = block_on(f);
        println!("fib({}) = {}", i, val);
    }

    let f1 = fib(20);
    let f2 = fib(10);

    let (a, b) = block_on(async_join(f1, f2));
    println!("fib(100) = {}, fib(10) = {}", a, b);
}
