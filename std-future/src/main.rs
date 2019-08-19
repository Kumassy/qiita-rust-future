#![feature(async_await, futures_api)]
extern crate tokio;
// use std::future::{Future, TryFutureExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::{Future, TryFutureExt};

struct MyFuture;
impl Future for MyFuture {
    type Output = String;
    // type Error = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("poll called");
        Poll::Ready("Done".to_string())
    }
}

// impl TryFutureExt for MyFuture {

// }


fn main() {
    // let future = (MyFuture{}).and_then(|res| {
    //     println!("{}", res);
    //     Ok(())
    // }).map_err(|err| {
    //     println!("{}", err); // () を返す
    // });

    // tokio::run(future);
    tokio::run(MyFuture{});
    println!("finished");
}