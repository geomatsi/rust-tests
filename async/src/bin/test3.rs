use {
    futures::{executor::block_on, join},
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
    for i in 1..10 {
        let f = FiboFuture::create(i);
        let v = block_on(f);
        println!("fib({}) = {}", i, v);
    }

    let f1 = FiboFuture::create(40);
    let f2 = FiboFuture::create(10);

    let (a, b) = block_on(async { join!(f1, f2) });
    println!("fib(100) = {}, fib(10) = {}", a, b);
}
