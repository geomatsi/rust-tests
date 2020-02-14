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
    Inactive,
    Running,
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
            StreamState::Inactive => {
                if shared_state.wake.is_none() {
                    shared_state.wake = Some(cx.waker().clone());
                }

                self.next_value();
                Poll::Pending
            }
            StreamState::Running => {
                if shared_state.wake.is_none() {
                    shared_state.wake = Some(cx.waker().clone());
                }

                Poll::Pending
            }
            StreamState::Done => {
                shared_state.state = StreamState::Inactive;
                Poll::Ready(Some(shared_state.result))
            }
            StreamState::Over => Poll::Ready(None),
        }
    }
}

impl FiboStream {
    pub fn create(max: usize, delay: u64) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            state: StreamState::Inactive,
            delay,
            wake: None,
            result: 0,
            max,
            id: 0,
        }));

        FiboStream { shared_state }
    }

    /* example is intentionally slow: calculates fib(id) each time from the start yielding between each step */
    fn next_value(&self) {
        let thread_shared_state = self.shared_state.clone();

        thread::spawn(move || {
            // lock mutex
            let mut shared_state = thread_shared_state.lock().unwrap();
            let ms = time::Duration::from_millis(shared_state.delay);

            let max = shared_state.max;
            let id = shared_state.id;

            if id > max {
                shared_state.state = StreamState::Over;

                if let Some(wake) = shared_state.wake.take() {
                    wake.wake()
                }

                return;
            }

            shared_state.state = StreamState::Running;
            // unlock mutex before long calculation that has no impact on internal state
            drop(shared_state);

            let result = match id {
                0 => 1,
                1 => 1,
                _ => {
                    let mut n1 = 0;
                    let mut n2 = 1;
                    let mut n;

                    for _ in 0..id {
                        n = n1;
                        n1 = n2;
                        n2 = n1 + n;
                        thread::sleep(ms);
                    }

                    n2
                }
            };

            // ready to report completion of computation: lock mutex again
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.state = StreamState::Done;
            shared_state.result = result;
            shared_state.id += 1;

            if let Some(wake) = shared_state.wake.take() {
                wake.wake()
            }
        });
    }
}

fn main() {
    /* simple step-by-step execution of stream in the loop */

    let mut s = FiboStream::create(10, 0);

    for i in 0..14 {
        let (res, _) = block_on(async { s.by_ref().into_future().await });
        if let Some(v) = res {
            println!("#1: step {} -> {}", i, v);
        } else {
            println!("#1: stream done");
            break;
        }
    }

    /* example with iterator */

    let s = FiboStream::create(10, 0);

    block_on(s.for_each(|v| {
        println!("#2: for_each -> {}", v);
        future::ready(())
    }));

    /* select method */

    let a = FiboStream::create(20, 1);
    let b = FiboStream::create(20, 10);

    block_on(stream::select(a, b).for_each(|v| {
        println!("#3: select -> {}", v);
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
                        println!("#4: a -> {}", v);
                    } else {
                        println!("#4: a -> done");
                    }
                },
                n = b.select_next_some() => {
                        println!("#4: b -> {:?}", n);
                },
                default =>  {},
                complete => break,
            }
        }
    });
}
