use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Runtime error: {0}.")]
    Runtime(#[from] rquickjs::Error),
    #[error("Job execute failed: {0}.")]
    JobException(String),
    #[error("unknown runtime error.")]
    Unknown,
}
