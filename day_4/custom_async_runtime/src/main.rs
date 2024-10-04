use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Wake},
    time::{Duration, Instant},
};

async fn do_stuff(name: &str) {
    use std::time::Duration;

    println!("{name:>5}: He...");
    sleep(Duration::from_secs(1)).await;
    println!("{name:>5}: ...llo...");
    sleep(Duration::from_secs(1)).await;
    println!("{name:>5}: ...world!");
}

fn sleep(duration: Duration) -> impl Future {
    SleepTimer {
        started_at: Instant::now(),
        duration,
    }
}

struct SleepTimer {
    started_at: Instant,
    duration: Duration,
}
impl Future for SleepTimer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.started_at.elapsed() > self.duration {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}

fn main() {
    let now = std::time::Instant::now();

    let mut rt = Runtime::new();

    rt.spawn(do_stuff("Alice"));
    rt.spawn(do_stuff("Bob"));

    rt.block_all();

    dbg!(now.elapsed());
}

struct Runtime {
    futures: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Runtime {
    fn new() -> Self {
        Runtime {
            futures: Vec::new(),
        }
    }

    fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.futures.push(Box::pin(future))
    }

    fn block_all(&mut self) {
        let waker = Arc::new(NoopWaker).into();
        let mut cx = Context::from_waker(&waker);

        let mut all_done = false;
        while !all_done {
            all_done = true;

            for i in (0..self.futures.len()).rev() {
                let poll_res = self.futures[i].as_mut().poll(&mut cx);
                match poll_res {
                    std::task::Poll::Ready(_) => drop(self.futures.remove(i)),
                    std::task::Poll::Pending => all_done = false,
                }
            }
        }
    }
}

struct NoopWaker;

impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {
        todo!()
    }
}
