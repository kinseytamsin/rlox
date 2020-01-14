use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
};

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

fn run(source: &[u8]) -> anyhow::Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // For now, just print the tokens.
    for token in tokens.iter() {
        println!("{:?}", token);
    }
    Ok(())
}

pub fn run_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    File::open(path.as_ref())?.read_to_end(&mut bytes)?;

    run(&bytes.into_boxed_slice())
}

pub fn run_prompt() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = Vec::<u8>::new();
        handle.read_until(b'\n', &mut line)?;
        let run_result = run(&line.into_boxed_slice());
        if let Err(e) = run_result {
            eprintln!("{}", e);
        }
    }
}
