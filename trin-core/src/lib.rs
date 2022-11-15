#![feature(map_first_last)]

pub mod cli;
pub mod jsonrpc;
pub mod locks;
pub mod portalnet;
pub mod socket;
pub mod types;
pub mod utils;
pub mod utp;

#[macro_use]
extern crate lazy_static;

pub const TRIN_VERSION: &str = env!("CARGO_PKG_VERSION");
