use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use anyhow;
use thiserror;

use crate::scanner::*;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("unterminated string")]
    UnterminatedString,
    #[error("unexpected character")]
    UnexpectedCharacter,
    #[error("IO error ({source})")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("internal parsing error ({source})")]
    Utf8Error {
        #[from]
        source: std::str::Utf8Error,
    },
    #[error("internal parsing error ({source})")]
    ParseFloatError {
        #[from]
        source: std::num::ParseFloatError,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("[line {line}] Error: {kind}")]
pub struct Error {
    line: usize,
    kind: ErrorKind,
}

impl Error {
    pub fn new(line: usize, kind: ErrorKind) -> Self {
        Self { line, kind }
    }
}

fn run(source: Vec<u8>) -> anyhow::Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // For now, just print the tokens.
    for token in tokens.into_iter() {
        println!("{:?}", token);
    }
    Ok(())
}

pub fn run_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    File::open(path.as_ref())?.read_to_end(&mut bytes)?;
    match run(bytes) {
        Ok(_)  => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn run_prompt() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        handle.read_line(&mut line)?;
        let run_result = run(line.into_bytes());
        if let Err(e) = run_result {
            eprintln!("{}", e);
        }
    }
}
