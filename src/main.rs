extern crate anyhow;
#[macro_use]
extern crate phf;
extern crate thiserror;

mod lox;
mod scanner;
mod token;

use std::cmp::Ordering;
use std::env;
use std::process;

use anyhow::Result;

fn main() -> Result<()> {
    let mut args_iter = env::args();
    let executable = args_iter.next().unwrap();
    let args: Vec<String> = args_iter.collect();

    match args.len().cmp(&1) {
        Ordering::Greater => {
            println!("Usage: {} [script]", executable);
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
