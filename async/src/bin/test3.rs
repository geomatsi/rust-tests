use {
    futures::{executor::block_on, join, pin_mut, select, FutureExt},
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
    },
};

pub struct FiboFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    done: bool,
    wake: Option<Waker>,
    input: u64,
    result: u64,
}

impl Future for FiboFuture {
    type Output = u64;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.done {
            Poll::Ready(shared_state.result)
        } else {
            shared_state.wake = Some(cx.waker().clone());
            self.start();
            Poll::Pending
        }
    }
}

impl FiboFuture {
    pub fn create(value: u64) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            done: false,
            wake: None,
            input: value,
            result: 0,
        }));

        FiboFuture { shared_state }
    }

    fn start(&self) {
        let thread_shared_state = self.shared_state.clone();
        thread::spawn(move || {
            let mut shared_state = thread_shared_state.lock().unwrap();

            let mut n1 = 0;
            let mut n2 = 1;

            for _ in 0..shared_state.input {
                n2 += n1;
                n1 = n2 - n1;
            }

            shared_state.result = n2;
            shared_state.done = true;

            if let Some(wake) = shared_state.wake.take() {
                wake.wake()
            }
        });
    }
}

fn main() {
    // block_on

    for i in 1..10 {
        let f = FiboFuture::create(i);
        let v = block_on(f);
        println!("fib({}) = {}", i, v);
    }

    // join

    let f1 = FiboFuture::create(40);
    let f2 = FiboFuture::create(10);

    let (a, b) = block_on(async { join!(f1, f2) });
    println!("fib(40) = {}, fib(10) = {}", a, b);

    // select: wait in loop for all completions

    let f3 = FiboFuture::create(70).fuse();
    let f4 = FiboFuture::create(50).fuse();
    let f5 = FiboFuture::create(30).fuse();

    pin_mut!(f3, f4, f5);

    block_on(async {
        loop {
            select! {
                n = f3 => println!("Fib(70) = {}", n),
                n = f4 => println!("Fib(50) = {}", n),
                n = f5 => println!("Fib(30) = {}", n),
                default => println!("not yet..."),
                complete => break,
            }
        }
    });

    // select: wait for the next completion

    let f6 = FiboFuture::create(71).fuse();
    let f7 = FiboFuture::create(51).fuse();
    let f8 = FiboFuture::create(31).fuse();

    pin_mut!(f6, f7, f8);

    block_on(async {
        select! {
            n = f6 => println!("Fib(71) = {}", n),
            n = f7 => println!("Fib(51) = {}", n),
            n = f8 => println!("Fib(31) = {}", n),
        }
    });

    block_on(async {
        select! {
            n = f6 => println!("Fib(71) = {}", n),
            n = f7 => println!("Fib(51) = {}", n),
            n = f8 => println!("Fib(31) = {}", n),
        }
    });

    block_on(async {
        select! {
            n = f6 => println!("Fib(71) = {}", n),
            n = f7 => println!("Fib(51) = {}", n),
            n = f8 => println!("Fib(31) = {}", n),
        }
    });
}
