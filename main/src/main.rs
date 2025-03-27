use std::{os::unix::process, string};

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use bytes::Bytes;
use std::sync::{Arc, Mutex, MutexGuard};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process( socket: TcpStream, db: Db, counter: &Arc<Mutex<i32>>) {
    // let mut db: HashMap<String, Vec<u8>> = HashMap::new();
    let mut connection = Connection::new(socket);

    {
        let counter:Result<MutexGuard<i32>, std::sync::PoisonError<MutexGuard<i32>>>  = counter.lock();
        if let Ok(mut counter_val) = counter {
            *counter_val = *counter_val + 1;
        }
    }

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


}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let shared_req_counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let shared_req_counter_th: Arc<Mutex<i32>> = shared_req_counter.clone();
        let shared_req_counter_rt: Arc<Mutex<i32>> = shared_req_counter.clone();

        // this is local vars
        let db = db.clone();
        
        tokio::spawn(async move {
            process(socket, db, &shared_req_counter_th).await;
        });

        let handle = tokio::spawn(async {
            "return value"
        });

        match handle.await {
            Ok(data) => {
                {
                    let shared_req_counter_ref = shared_req_counter_rt.lock().unwrap();
                    println!("returned val conn {}: {}", shared_req_counter_ref, data.to_string());
                   
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }


    }
}
