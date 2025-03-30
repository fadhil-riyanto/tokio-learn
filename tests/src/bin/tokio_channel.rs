use rand::{Rng, SeedableRng};
use std::num::NonZeroU64;
use tokio::sync::mpsc;
use rand::rngs::StdRng;

#[derive(Debug)]
struct DataToSend {
        task_id: tokio::task::Id,
        data: u32,
}

#[tokio::main]
async fn main() {
        let (tx, mut rx): (mpsc::Sender<DataToSend>, mpsc::Receiver<DataToSend>) =
                mpsc::channel(32);

        let sender = tokio::spawn(async move {
                // let mut seed = rand::rng();
                let id: tokio::task::Id = tokio::task::id();
                let mut rng = rand::rngs::SmallRng::from_os_rng(); // generate enthropy
                
                loop {
                        let num = rng.random_range(0..100);
                        let data = DataToSend {
                                task_id: id,
                                data: num,
                        };
        
                        let _ = tx.send(data).await;
                }
                // println!("current id {}", res);
                // impl Future<Output = Result<..., ...>>
        });

        let reader = tokio::spawn(async move {
                // let mut seed = rand::rng();
                loop {
                        let data = rx.recv().await;

                        if let Some(data) = data {
                                println!("{:?}", data);
                        }
                }
                // println!("current id {}", res);
                // impl Future<Output = Result<..., ...>>
        });

        sender.await;
        reader.await;
}

// use rand::{Rng, SeedableRng};
// use tokio::sync::mpsc;
// use tokio::task;
// use rand::rngs::StdRng;
// use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel(32);

//     // Spawn a task to generate random numbers
//     let generator = task::spawn(async move {
//         let mut rng = StdRng::from_os_rng();

//         loop {
//             let num = rng.random_range(0..100);
//             if tx.send(num).await.is_err() {
//                 println!("Receiver dropped, stopping generator.");
//                 break;
//             }
//             tokio::time::sleep(Duration::from_secs(1)).await;
//         }
//     });

//     // Spawn multiple receiver tasks
//     for i in 1..=3 {
//         // let mut rx_clone = rx.clone();
//         task::spawn(async move {
//             while let Some(num) = rx.recv().await {
//                 println!("Receiver {} received: {}", i, num);
//             }
//         });
//     }

//     // Allow the generator to run for a while
//     tokio::time::sleep(Duration::from_secs(10)).await;
// //     drop(rx);
//     generator.await.unwrap();
// }
