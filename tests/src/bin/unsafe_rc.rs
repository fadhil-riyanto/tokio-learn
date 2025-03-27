use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // force drop rc
        {
            let rc = Rc::new("data");
            println!("{}", rc);
        }

        yield_now().await;
    });
}
