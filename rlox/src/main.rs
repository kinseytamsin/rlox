use std::{
    cmp::Ordering,
    env,
    process
};

use anyhow::Result;

mod lox;
mod scanner;
mod token;

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
