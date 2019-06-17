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
//! Should generate a struct + trait implementation for the NegiTask
//!
//!

#[typetag::serde(tag = "type")]
pub trait Task: Send + Sync {
    fn execute(&self);
}
