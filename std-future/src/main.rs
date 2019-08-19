use std::future::{Future};
use std::pin::Pin;
use std::task::{Context};

extern crate futures;
use futures::{Poll, TryFutureExt};
use futures::executor::{block_on, ThreadPool};

extern crate rand;
use rand::Rng;

struct MyFuture;
impl Future for MyFuture {
    type Output = Result<String, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("poll called");
        
        let mut rng = rand::thread_rng();
        let i: i32 = rng.gen_range(0, 10);
        if i < 5 {
            Poll::Ready(Ok("success".to_string()))
        } else {
            Poll::Ready(Err("failure".to_string()))
        }
    }
}

fn get_future() -> impl Future<Output=Result<(), ()>> {
    (MyFuture{}).map_ok(|res| {
        println!("{}", res);
    }).map_err(|err| {
        println!("{}", err);
    })
}

fn main() {
    let future = (MyFuture{}).map_ok(|res| {
        println!("{}", res);
    }).map_err(|err| {
        println!("{}", err);
    });
    let _ = block_on(future);

    let future = get_future();
    ThreadPool::new().expect("Failed to create threadpool").run(future);


    let future = (MyFuture{}).inspect_ok(|res| {
        println!("{}", res); 
    }).inspect_err(|err| {
        println!("{}", err);
    });

    let _ = block_on(future);
}