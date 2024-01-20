#![warn(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]

mod cache;
pub mod error;
#[macro_use]
mod util;
mod header;
pub mod packet;
mod question;
mod record;
pub mod resolver;
pub mod rr_fields;
