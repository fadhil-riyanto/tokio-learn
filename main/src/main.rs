use std::{os::unix::process, string};

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use bytes::Bytes;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process( socket: TcpStream, db: Db) {
    // let mut db: HashMap<String, Vec<u8>> = HashMap::new();
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();

                if let Some(data) = db.get(cmd.key()) {
                    Frame::Bulk(data.clone())
                } else {
                    Frame::Null
                }
                // Frame::Simple("OK".to_string())
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }

    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("got data {:?}", frame);

    //     let response = Frame::Error("uninplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

    // let mut buf = [65];
    // socket.write(&buf).await;
    // println!("data {:?}", connection.read_frame().await.unwrap());
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // this is local vars
        let db = db.clone();
        
        tokio::spawn(async move {
            process(socket, db).await;
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
