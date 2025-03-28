#![feature(thread_id_value)]

use rand::prelude::*;
use std::sync::mpsc;
use std::thread;

struct DataToSend {
        thread_id: std::num::NonZero<u64>,
        data: u32,
}

impl std::fmt::Debug for DataToSend {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                        f,
                        "DataToSend {{ thread_id: {:?}, data: {} }}",
                        self.thread_id, self.data
                )
        }
}

#[inline(always)]
fn send(tx: mpsc::Sender<DataToSend>) {
        let mut seed = rand::rng();

        for _ in 0..100000 {
                let _ = tx.send(DataToSend {
                        thread_id: thread::current().id().as_u64(),
                        data: seed.random::<u32>(),
                });
        }
}

#[inline(always)]
fn recv(tx: mpsc::Receiver<DataToSend>) {
        for _ in 0..100000 {
                let data = tx.recv();
                if let Ok(data_val) = data {
                        println!("{:?}", data_val);
                }
        }
}

fn main() {
        let (tx, rx): (mpsc::Sender<DataToSend>, mpsc::Receiver<DataToSend>) = mpsc::channel();

        let t1 = thread::spawn(|| {
                send(tx);
        });

        let t2 = thread::spawn(|| {
                recv(rx);
        });

        let _ = t1.join();
        let _ = t2.join();
}
