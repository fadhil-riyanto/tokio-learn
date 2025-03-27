use tokio::task::yield_now;
use std::rc::Rc;

pub async fn drop0() -> () {
    tokio::spawn(async {
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }
    });
}