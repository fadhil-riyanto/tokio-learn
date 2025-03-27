async fn world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    let op = world();

    println!("hello");
    op.await;
}