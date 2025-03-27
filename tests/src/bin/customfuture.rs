use std::future::{self, Future};
use std::pin::Pin;
use std::task::Context;
use std::default::Default;
use std::task::Poll;
use rand::prelude::*;

#[derive(Default)]
struct RandFuture;

impl Future for RandFuture {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut rng = rand::rng();

        Poll::Ready(rng.random::<Self::Output>())
    }
}

#[tokio::main]
async fn main() {
    println!("running");
    let rand = RandFuture {};
    let out = rand.await;

    println!("out {out}");
}