use tokio::task;
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");

    Ok(())
}
