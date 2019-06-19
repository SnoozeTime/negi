use negi_core::{Task, DEFAULT_TOPIC};
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

    tokio::run(
        connect
            .and_then(|c| {
                let task = MyCustom {
                    some_string: "hi".to_owned(),
                    x: 1231,
                };
                let packet = ser(&task);

                redis::cmd("RPUSH")
                    .arg(DEFAULT_TOPIC)
                    .arg(packet)
                    .query_async::<_, i32>(c)
            })
            .and_then(|(_, res)| {
                println!("RES => {:?}", res);
                futures::future::ok(())
            })
            .map_err(|e| {
                println!("{:?}", e);
            }),
    );
}
