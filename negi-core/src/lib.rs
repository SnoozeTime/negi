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

#[macro_use]
extern crate failure;

mod error;
mod redis;

pub use crate::redis::*;

pub use error::Error;
use futures::Future;

#[typetag::serde(tag = "type")]
pub trait Task: Send + Sync {
    fn execute(&self);
}
/// Structure used to send tasks to the dispatcher.
pub struct Client<B: Backend> {
    pub backend: B,
}

impl<B: Backend> Client<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn send(&self, task: &dyn Task) -> Result<(), Error> {
        self.backend.send(task)
    }

    pub fn send_async(&self, task: &dyn Task) -> Box<Future<Item = (), Error = Error> + Send> {
        self.backend.send_async(task)
    }
}

pub trait Backend: Send + Sync {
    fn send(&self, t: &dyn Task) -> Result<(), Error>;
    fn send_async(&self, t: &dyn Task) -> Box<Future<Item = (), Error = Error> + Send>;
}
