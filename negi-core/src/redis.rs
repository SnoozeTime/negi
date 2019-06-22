//! Queue implementation using Redis
//!
//! The system that wants to send tasks need to create a client.
//! ```
//! let client = BackendBuilder::new().build()?;
//! let task = SomeTask {};
//!
//! // Can send synchronously
//! client.send(&task)?;
//!
//! // Can send asynchronously
//! tokio::run(client.send_async(&task).map_err(|e| println!("{:?}", e)));
//!
//! ```
//!
//! A dispatcher needs to be started in order to execute the tasks. This can
//! be done with:
//!
//! ```
//! let dispatcher = Dispatcher::new("redis://127.0.0.1".to_string(), "topic".to_string());
//! dispatcher.run()
//! ```

use futures::Future;

use crate::{Backend, Error, Task};
pub const DEFAULT_TOPIC: &'static str = "negi_topic";
pub const LOCAL_URL: &'static str = "redis://127.0.0.1";

pub struct RedisBackend {
    inner: ::redis::Client,
    topic: String,
}

pub struct BackendBuilder {
    topic: String,
    url: String,
}

impl std::default::Default for BackendBuilder {
    fn default() -> Self {
        Self {
            topic: DEFAULT_TOPIC.to_owned(),
            url: LOCAL_URL.to_owned(),
        }
    }
}

impl BackendBuilder {
    pub fn new() -> Self {
        Self {
            topic: DEFAULT_TOPIC.to_owned(),
            url: LOCAL_URL.to_owned(),
        }
    }

    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = topic.to_owned();
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_owned();
        self
    }

    pub fn build(&self) -> Result<crate::Client<RedisBackend>, crate::Error> {
        RedisBackend::connect(self.url.as_str(), self.topic.clone()).map(|b| crate::Client::new(b))
    }
}

impl RedisBackend {
    pub fn connect(url: &str, topic: String) -> Result<Self, Error> {
        let inner = ::redis::Client::open(url)?;
        Ok(Self { inner, topic })
    }
}

impl Backend for RedisBackend {
    fn send(&self, t: &dyn Task) -> Result<(), Error> {
        Ok(())
    }

    fn send_async(&self, t: &dyn Task) -> Box<Future<Item = (), Error = Error> + Send> {
        let packet = serde_json::to_string(t).unwrap();
        Box::new(
            self.inner
                .get_async_connection()
                .and_then(move |conn| {
                    ::redis::cmd("RPUSH")
                        .arg(DEFAULT_TOPIC)
                        .arg(packet)
                        .query_async::<_, i32>(conn)
                })
                .and_then(|(_, _)| futures::future::ok(()))
                .map_err(|e| e.into()),
        )
    }
}

/// Dispatcher will connect to redis and listen for new task on the list `topic`.
/// It will send the tasks to a thread pool for execution.
///
/// TODO:
/// - Abstract dispatcher from redis
/// - Builder for more flexible configuration (number of thread for example)
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
    pub fn run(&self) -> Result<(), ::redis::RedisError> {
        // Will execute the tasks in this thread pool.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

        let client = ::redis::Client::open(self.redis_url.as_str())?;
        let con: ::redis::Connection = client.get_connection()?;
        'dispatch_loop: loop {
            // BLPOP is blocking. It will not return until element is
            // pushed to self.topic.
            let (_, task): (String, String) = ::redis::cmd("BLPOP")
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
