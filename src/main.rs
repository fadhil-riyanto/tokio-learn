use std::os::unix::process;

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            process(socket).await;
        });

        let handle = tokio::spawn(async {
            "return value"
        });

        match handle.await {
            Ok(data) => {
                println!("returned val: {}", data.to_string());
            },
            Err(err) => {
                println!("{}", err);
            }
        }


    }
}

async fn process( socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("got data {:?}", frame);

        let response = Frame::Error("uninplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

    // let mut buf = [65];
    // socket.write(&buf).await;
    // println!("data {:?}", connection.read_frame().await.unwrap());
}