#![forbid(unsafe_code)]
use clap::{arg, command};
use test::{example_node_main,Params};
use tracing::Level;
use tracing_subscriber::prelude::*;

#[cfg(feature = "dlt")]
use dlt_tracing_subscriber::DltLayer;

fn main() {
    println!("Hello, world!");
}
