use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling...");

        if self.count < 5 {
            self.count += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let future = MyFuture { count: 0 };

    println!("Starting the runtime...");

    let result = future.await;
    println!("Result: {}", result);

    // Simulating some other async tasks
    let _ = sleep(Duration::from_secs(2)).await;
    println!("Other task completed.");

    println!("Runtime finished.");
}