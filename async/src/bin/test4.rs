use futures::StreamExt;
use {
    futures::{executor::block_on, future, select, stream, stream::Stream},
    std::{
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread, time,
    },
};

pub struct FiboStream {
    shared_state: Arc<Mutex<SharedState>>,
}

enum StreamState {
    Wait,
    Done,
    Over,
}

struct SharedState {
    wake: Option<Waker>,
    state: StreamState,
    delay: u64,
    result: u64,
    max: usize,
    id: usize,
}

impl Stream for FiboStream {
    type Item = u64;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match shared_state.state {
            StreamState::Wait => {
                shared_state.wake = Some(cx.waker().clone());
                self.next_value();
                Poll::Pending
            }
            StreamState::Done => {
                shared_state.state = StreamState::Wait;
                Poll::Ready(Some(shared_state.result))
            }
            StreamState::Over => Poll::Ready(None),
        }
    }
}

impl FiboStream {
    pub fn create(max: usize, delay: u64) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            state: StreamState::Wait,
            delay,
            wake: None,
            result: 0,
            max,
            id: 0,
        }));

        FiboStream { shared_state }
    }

    /* example is intentionally slow: calculates fib(id) each time from the start with extra delay */
    fn next_value(&self) {
        let thread_shared_state = self.shared_state.clone();
        thread::spawn(move || {
            let mut shared_state = thread_shared_state.lock().unwrap();
            let ms = time::Duration::from_millis(shared_state.delay);

            if shared_state.id > shared_state.max {
                shared_state.state = StreamState::Over;
            } else {
                shared_state.result = match shared_state.id {
                    0 => 1,
                    1 => 1,
                    _ => {
                        let mut n1 = 0;
                        let mut n2 = 1;
                        let mut n;

                        for _ in 0..shared_state.id {
                            n = n1;
                            n1 = n2;
                            n2 = n1 + n;
                            thread::sleep(ms);
                        }

                        n2
                    }
                };

                shared_state.state = StreamState::Done;
                shared_state.id += 1;
            }

            if let Some(wake) = shared_state.wake.take() {
                wake.wake()
            }
        });
    }
}

fn main() {
    /* simple execution in the loop */

    let mut s = FiboStream::create(10, 0);

    for _ in 0..12 {
        let (res, _) = block_on(async { s.by_ref().into_future().await });
        if let Some(v) = res {
            println!("-> {}", v);
        } else {
            println!("done");
        }
    }

    /* example with iterator */

    let s = FiboStream::create(10, 0);

    block_on(s.for_each(|v| {
        println!("-> {}", v);
        future::ready(())
    }));

    /* select method */

    let a = FiboStream::create(20, 1);
    let b = FiboStream::create(20, 10);

    block_on(stream::select(a, b).for_each(|v| {
        println!("select -> {}", v);
        future::ready(())
    }));

    /* select macro */

    let mut a = FiboStream::create(10, 1).fuse();
    let mut b = FiboStream::create(20, 10).fuse();

    block_on(async {
        loop {
            select! {
                n = a.next() => {
                    if let Some(v) = n {
                        println!("a -> {}", v);
                    } else {
                        println!("a -> done");
                    }
                },
                n = b.select_next_some() => {
                        println!("b -> {:?}", n);
                },
                default => println!("not yet..."),
                complete => break,
            }
        }
    });
}
