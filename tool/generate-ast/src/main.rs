use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufWriter, prelude::*},
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    str::FromStr,
};

use anyhow::Result;
use heck::SnakeCase;
use lazy_static::lazy_static;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

macro_rules! ident {
    ($x: expr) => {
        Ident::new($x, Span::call_site())
    }
}

const BINARY_NAME: &str = "generate-ast";

fn define_kind(
    tokens: &mut TokenStream,
    base_name: &str,
    kind_name: &str,
    field_list: &str,
) -> Result<()> {
    let struct_name = ident!(&format!("{}{}", base_name, kind_name));
    let fields = field_list.split(", ");
    let field_names =
        fields.clone().map(|field| ident!(field.split(": ").nth(0).unwrap()));
    let struct_fields = TokenStream::from_str(field_list).unwrap();
    let new_args =
        fields.clone().map(|field| TokenStream::from_str(field).unwrap());
    tokens.extend(quote! {
        struct #struct_name {
            #struct_fields
        }

        impl #struct_name {
            fn new(#(#new_args),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    });
    Ok(())
}

fn define_ast<P: AsRef<Path>>(
    output_dir: P,
    base_name: &str,
    kinds: &HashMap<&str, &str>,
) -> Result<()> {
    let mut tokens = TokenStream::new();

    let base_name_ident = ident!(base_name);
    let kind_names = kinds.keys().map(|x| ident!(x));
    let struct_names =
        kinds.keys().map(|kind| ident!(&format!("{}{}", base_name, kind)));

    tokens.extend(quote! {
        use crate::token::*;

        enum #base_name_ident {
            #( #kind_names(#struct_names) ),*
        }
    });
    for (&kind_name, &fields) in kinds.iter() {
        define_kind(&mut tokens, base_name, kind_name, fields)?;
    }
    let code = tokens.to_string();

    let mut rustfmt =
        Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
    {
        let mut stdin = rustfmt.stdin.take().unwrap();
        stdin.write_all(&code.into_bytes())?;
        stdin.flush()?;
    }
    let formatted_code = rustfmt.wait_with_output()?.stdout;

    let mut path: PathBuf = output_dir.as_ref().to_owned();
    path.push(format!("{}.rs", base_name.to_snake_case()));
    let f = File::create(&path)?;
    {
        let mut buffer = BufWriter::new(f);
        buffer.write_all(&formatted_code)?;
        buffer.flush()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    lazy_static! {
        static ref KINDS: HashMap<&'static str, &'static str> = {
            let mut m = HashMap::new();
            m.insert("Binary",   "left: Expr, operator: Token, right: Expr");
            m.insert("Grouping", "expression: Expr");
            m.insert("Literal",  "value: Literal");
            m.insert("Unary",    "operator: Token, right: Expr");
            m
        };
    }
    let args = env::args().skip(1).collect::<Box<_>>();
    if args.len() != 1 {
        eprintln!("Usage: {} <output directory>", BINARY_NAME);
        process::exit(1);
    }
    let output_dir = &args[0];
    define_ast(output_dir, "Expr", &KINDS)?;
    Ok(())
}
