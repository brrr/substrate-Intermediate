//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
// mod benchmarking;
mod cli;
mod command;
mod rpc;
mod node_primitives;
use cli::*;

fn main() -> sc_cli::Result<()> {
	command::run()
}
