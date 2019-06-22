use negi_core::{Task, DEFAULT_TOPIC, BackendBuilder};
use tokio::prelude::*;

use negi_macro::task;

#[task(module)]
fn MyCustom(some_string: String, x: u32) {
    println!("HHHHHH {} + {}", some_string, x);
}

fn ser(event: &dyn Task) -> String {
    serde_json::to_string(event).unwrap()
}
fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let connect = client.get_async_connection();

    let c = BackendBuilder::new().build().unwrap();
    tokio::run(
        futures::lazy(move || {
            let task = MyCustom {
                some_string: "hi".to_owned(),
                x: 1231,
            };

            c.send_async(&task)
                .map_err(|e| {
                    println!("{:?}", e);
                })
        })
    );
}
