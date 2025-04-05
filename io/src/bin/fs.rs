use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut fd = File::open("sample.txt").await?;
    let mut buf: [u8; 4096] = [0; 4096]; 

    let n = fd.read(&mut buf).await?;

    for i in buf {
        print!("{}", i as char);
    }

    Ok(())
}