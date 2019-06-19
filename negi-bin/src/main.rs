use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::QueryableByName;
use negi_core::{Dispatcher, Task};
use serde_derive::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

use negi_macro::task;

#[task]
fn MyCustom(some_string: String, x: u32) {
    println!("HHHHHH {} + {}", some_string, x);
}

// ---------------------------------

fn dispatch() -> Result<(), redis::RedisError> {
    // Will execute the tasks in this thread pool.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: redis::Connection = client.get_connection()?;
    'dispatch_loop: loop {
        let task: Option<String> = redis::cmd("LPOP").arg("negi_msg").query(&con)?;

        if let Some(task) = task {
            let task: serde_json::Result<Box<dyn Task>> = serde_json::from_str(&task);
            match task {
                Ok(task) => {
                    pool.spawn(move || task.execute());
                    println!("hi");
                }
                Err(err) => println!("Error {:?}", err),
            }
        }
        // Don't use my CPU.
        std::thread::sleep(Duration::from_millis(100));
    }
}
//
//fn ser(event: &dyn Task) -> String {
//    serde_json::to_string(event).unwrap()
//}
//
//fn deser(payload: String) -> Box<dyn Task> {
//    serde_json::from_str(&payload).unwrap()
//}
//
fn main() -> Result<(), redis::RedisError> {
    //deser(encoded.unwrap()).inspect();
    let dispatcher = Dispatcher::default();
    dispatcher.run()
}
