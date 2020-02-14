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

enum FiboStatus {
    Inactive,
    Running,
    Done,
}

struct SharedState {
    status: FiboStatus,
    wake: Option<Waker>,
    input: u64,
    result: u64,
    polled: u64,
}

impl Future for FiboFuture {
    type Output = (u64, u64);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        match shared_state.status {
            FiboStatus::Inactive => {
                if shared_state.wake.is_none() {
                    shared_state.wake = Some(cx.waker().clone());
                }

                self.start();
                Poll::Pending
            }
            FiboStatus::Running => {
                if shared_state.wake.is_none() {
                    shared_state.wake = Some(cx.waker().clone());
                }

                shared_state.polled += 1;
                Poll::Pending
            }
            FiboStatus::Done => Poll::Ready((shared_state.result, shared_state.polled)),
        }
    }
}

impl FiboFuture {
    pub fn create(value: u64) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            status: FiboStatus::Inactive,
            wake: None,
            input: value,
            result: 0,
            polled: 0,
        }));

        FiboFuture { shared_state }
    }

    fn start(&self) {
        let thread_shared_state = self.shared_state.clone();

        thread::spawn(move || {
            // lock mutex
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.status = FiboStatus::Running;

            let n = shared_state.input;
            let mut n1 = 0;
            let mut n2 = 1;

            // drop to unlock mutex
            drop(shared_state);

            for _ in 0..n {
                n2 += n1;
                n1 = n2 - n1;

                // lock mutex
                let mut shared_state = thread_shared_state.lock().unwrap();

                if let Some(wake) = shared_state.wake.take() {
                    wake.wake();
                }

                // drop to unlock mutex
                drop(shared_state);

                // the chance for other threads to run
                thread::yield_now();
            }

            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.result = n2;
            shared_state.status = FiboStatus::Done;

            if let Some(wake) = shared_state.wake.take() {
                wake.wake();
            }
        });
    }
}

fn main() {
    // block_on

    for i in 1..10 {
        let f = FiboFuture::create(i);
        let v = block_on(f);
        println!("block_on: fib({}) = {} (polled {} times)", i, v.0, v.1);
    }

    // join

    let f1 = FiboFuture::create(40);
    let f2 = FiboFuture::create(10);

    let (a, b) = block_on(async { join!(f1, f2) });
    println!(
        "block_on + join: fib(40) = {} (polled {} times), fib(10) = {}, (polled {} times)",
        a.0, a.1, b.0, b.1
    );

    // select: wait in loop for all completions

    let f3 = FiboFuture::create(70).fuse();
    let f4 = FiboFuture::create(50).fuse();
    let f5 = FiboFuture::create(30).fuse();

    pin_mut!(f3, f4, f5);

    block_on(async {
        loop {
            select! {
                n = f3 => println!("select all: Fib(70) = {} (polled {} times)", n.0, n.1),
                n = f4 => println!("select all: Fib(50) = {} (polled {} times)", n.0, n.1),
                n = f5 => println!("select all: Fib(30) = {} (polled {} times)", n.0, n.1),
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
            n = f6 => println!("select next: Fib(71) = {} (polled {} times)", n.0, n.1),
            n = f7 => println!("select next: Fib(51) = {} (polled {} times)", n.0, n.1),
            n = f8 => println!("select next: Fib(31) = {} (polled {} times)", n.0, n.1),
        }
    });

    block_on(async {
        select! {
            n = f6 => println!("select next: Fib(71) = {} (polled {} times)", n.0, n.1),
            n = f7 => println!("select next: Fib(51) = {} (polled {} times)", n.0, n.1),
            n = f8 => println!("select next: Fib(31) = {} (polled {} times)", n.0, n.1),
        }
    });

    block_on(async {
        select! {
            n = f6 => println!("select next: Fib(71) = {} (polled {} times)", n.0, n.1),
            n = f7 => println!("select next: Fib(51) = {} (polled {} times)", n.0, n.1),
            n = f8 => println!("select next: Fib(31) = {} (polled {} times)", n.0, n.1),
        }
    });
}
