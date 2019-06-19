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
use std::default::Default;

pub const DEFAULT_TOPIC: &'static str = "negi_topic";
pub const LOCAL_URL: &'static str = "redis://127.0.0.1";

#[typetag::serde(tag = "type")]
pub trait Task: Send + Sync {
    fn execute(&self);
}

pub struct Dispatcher {
    topic: String,
    redis_url: String,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self {
            topic: DEFAULT_TOPIC.to_owned(),
            redis_url: LOCAL_URL.to_owned(),
        }
    }
}

impl Dispatcher {
    /// Create a task dispatcher that will listen to `topic` list
    /// on redis
    pub fn new(redis_url: String, topic: String) -> Self {
        Self { topic, redis_url }
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

        let client = redis::Client::open(self.redis_url.as_str())?;
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
