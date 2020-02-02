use futures::executor::block_on;
use std::{thread, time};

async fn delay(s: &str) {
    let d = time::Duration::from_millis(100);

    for _ in 0..5 {
        thread::sleep(d);
        println!("{}: delay: {:?} msec", s, d);
    }
}

async fn async_join(t1: impl futures::future::Future, t2: impl futures::future::Future) {
    futures::join!(t1, t2);
}

async fn async_await(t: impl futures::future::Future) {
    t.await;
}

fn main() {
    let f1 = delay("test1");
    let f2 = delay("test2");
    let f3 = delay("test3");
    let f4 = delay("test4");
    let f5 = delay("test5");

    println!("block_on:");
    block_on(f1);

    println!("join:");
    block_on(async_join(f2, f3));

    println!("await:");
    block_on(async { f4.await });

    println!("async fn:");
    block_on(async_await(f5));
}
