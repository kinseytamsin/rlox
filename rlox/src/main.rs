extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate thiserror;

mod lox;
mod scanner;
mod token;

use std::cmp::Ordering;
use std::env;
use std::process;

use anyhow::Result;

const BINARY_NAME: &str = "rlox";

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Box<_>>();

    match args.len().cmp(&1) {
        Ordering::Greater => {
            println!("Usage: {} [script]", BINARY_NAME);
            process::exit(64);
        }
        Ordering::Equal => {
            lox::run_file(&args[0])?;
        }
        _ => {
            lox::run_prompt()?;
        }
    }
    Ok(())
}
