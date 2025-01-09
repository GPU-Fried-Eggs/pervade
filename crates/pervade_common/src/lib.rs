#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;

mod messages;
mod value;

pub use messages::{ClientMessage, ServerMessage, TaskStatus};
pub use value::Value;