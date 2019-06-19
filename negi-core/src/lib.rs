//! Negi abstracts task dispatch in a similar way as Celery.
//! Still experimental.
//!
//! ```rust
//! #[task]
//! fn some_task(arg: String) {
//!     // Do some task things.
//!
//! }
//!
//! ```
//!
//! Should generate a struct + trait implementation for Task
//!
//!
use futures::Future;
use redis::Cmd;
use std::default::Default;

pub const DEFAULT_TOPIC: &'static str = "negi_topic";

#[typetag::serde(tag = "type")]
pub trait Task: Send + Sync {
    fn execute(&self);
}

pub struct Dispatcher {
    topic: String,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self {
            topic: DEFAULT_TOPIC.to_owned(),
        }
    }
}

impl Dispatcher {
    /// Create a task dispatcher that will listen to `topic` list
    /// on redis
    pub fn new(topic: String) -> Self {
        Self { topic }
    }

    /// Will run the dispatcher. This is a blocking call as it will
    /// process incoming message on the redis topic (key).
    ///
    /// Process is popping items from the
    pub fn run(&self) -> Result<(), redis::RedisError> {
        // Will execute the tasks in this thread pool.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con: redis::Connection = client.get_connection()?;
        'dispatch_loop: loop {
            // BLPOP is blocking. It will not return until element is
            // pushed to self.topic.
            let (_, task): (String, String) = redis::cmd("BLPOP")
                .arg(self.topic.as_str())
                .arg("0")
                .query(&con)?;

            let task: serde_json::Result<Box<dyn Task>> = serde_json::from_str(&task);
            match task {
                Ok(task) => {
                    pool.spawn(move || task.execute());
                    println!("hi");
                }
                Err(err) => println!("Error {:?}", err),
            }
        }
    }
}

pub struct Client<B: Send + Sync> {
    pub backend: B,
}

pub type RedisClient = Client<redis::Client>;

use std::thread;
pub fn send(client: &redis::Client) -> impl Future<Item = (), Error = redis::RedisError> {
    client
        .get_async_connection()
        .and_then(|c| {
            redis::cmd("RPUSH")
                .arg(DEFAULT_TOPIC)
                .arg("HI")
                .query_async::<_, String>(c)
        })
        .and_then(|(conn, _)| futures::future::ok(()))
}

fn lol() {
    let c = redis::Client::open("localhost").unwrap();
    thread::spawn(move || {
        send(&c);
    });
}
