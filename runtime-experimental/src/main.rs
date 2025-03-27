use tokio::task::yield_now;
use std::rc::Rc;

mod tests;

#[tokio::main]
async fn main() {
    tests::drop0::drop0().await;
    let data = tests::future0::ft0();
    println!("{:?}", data);
}