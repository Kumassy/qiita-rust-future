#![feature(async_await, futures_api)]
// extern crate tokio;
// use std::future::{Future, TryFutureExt};
use std::pin::Pin;
use std::task::{Context};
extern crate futures;
use futures::{Future, TryFuture, Poll, TryFutureExt, FutureExt};
use futures::executor::{block_on, ThreadPool};

// #[derive(Debug, TryFutureExt)]
struct MyFuture;
impl TryFuture for MyFuture {
    type Ok = String;
    type Error = String;

    fn try_poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<Self::Ok, Self::Error>> {
        println!("poll called");
        Poll::Ready("Done".to_string())
    }
}
// impl FutureExt for MyFuture{}

// impl TryFutureExt for MyFuture {

// }


fn main() {
    let future = (MyFuture{}).map_ok(|res| {
        println!("{}", res);
        Ok(())
    }).map_err(|err| {
        println!("{}", err); // () を返す
    });

    // tokio::run(future);
    // tokio::run(MyFuture{});
    block_on(MyFuture{});
    ThreadPool::new().expect("Failed to create threadpool").run(MyFuture{});
    ThreadPool::new().expect("Failed to create threadpool").run(future);
    println!("finished");
}