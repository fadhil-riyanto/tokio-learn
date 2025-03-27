use tokio::task::{self, JoinError};

#[tokio::main]
async fn main() -> Result<(), JoinError>{
    let join = task::spawn(async {
        "hello world"
    });

    let res = join.await?;
    println!("{}", res);

    Ok(())
}