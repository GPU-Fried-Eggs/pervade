//! A minimal JavaScript runtime wrapper for WebAssembly.
//!
//! ```
//! use pervade_runtime::qjs::Function;
//! use pervade_runtime::Runtime;
//!
//! let runtime = Runtime::default();
//! let context = runtime.context();
//!
//! context.with(|ctx| {
//!     let func = Function::new(ctx.clone(), || "Hello world!").unwrap();
//!     ctx.globals().set("hello", func).unwrap();
//! });
//!
//! context.with(|ctx| {
//!     let content: String = ctx.eval("hello();").unwrap();
//!     assert_eq!(content, "Hello world!");
//! });
//! ```

mod config;
mod error;
mod runtime;

pub use config::{Config, JSIntrinsics};
pub use error::Error;
pub use rquickjs as qjs;
pub use runtime::Runtime;
