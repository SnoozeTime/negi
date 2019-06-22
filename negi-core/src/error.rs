//! Return values from backend functions
//!

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Redis Error happened: {}", error)]
    RedisError { error: redis::RedisError },
}

impl From<redis::RedisError> for Error {
    fn from(redis_err: redis::RedisError) -> Self {
        Error::RedisError { error: redis_err }
    }
}
