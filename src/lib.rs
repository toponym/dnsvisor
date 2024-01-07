#![warn(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]

mod cache;
pub mod error;
mod header;
mod packet;
mod query;
mod question;
mod record;
pub mod resolver;
pub mod rr_fields;
mod util;
