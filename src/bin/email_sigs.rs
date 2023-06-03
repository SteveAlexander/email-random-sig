/*
    TODO:
        - add logging / tracing
        - debug logging of what file types detected, and what errors
        - debug logging of how many quotes and how many duplicates, and number of anon vs attributed
        - debug logging of number of signatures erased

   syntax:

   email-random-sig --prefix="Autoquote: " --erase-all filename filename

   prefix is optional, and defaults to "Autoquote: "
   commands:
       erase-all : erase all sigs with prefix

   filenames are parsed, quotes combined in order, but made unique
*/

use email_sigs::quotes_parser;
use email_sigs::signatures::{add_signatures, erase_all_signatures, Signature};
use indexmap::IndexSet;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Args {
    help: bool,
    erase_all: bool,
    prefix: String,
    filenames: Vec<OsString>,
}

fn default_prefix() -> String {
    "Autoquote ".into()
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut help = false;
    let mut erase_all = false;
    let mut prefix = default_prefix();
    let mut filenames = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => help = true,
            Long("erase-all") => erase_all = true,
            Long("prefix") => prefix = parser.value()?.string()?,
            Value(val) => filenames.push(val),
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Args {
        help,
        erase_all,
        prefix,
        filenames,
    })
}

fn format_quote(quote: &quotes_parser::Quote) -> String {
    match &quote.author {
        None => format!("{}", quote.text),
        Some(author) => format!("{} - {}", quote.text, author),
    }
}

fn format_key(quote: &quotes_parser::Quote) -> String {
    match &quote.author {
        None => String::from("Anonymous"),
        Some(author) => author.clone(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    if args.help {
        println!(
r#"email_sigs: Update Mail.app email signatures

Signatures are updated for all accounts, with a prefixed id.
The default prefix is <{}>, but this can be overridden.

Usage:
  email_sigs -h | --help
  email_sigs --prefix "custom prefix "
  email_sigs FILENAME1 FILENAME2 FILENAME3
  email_sigs --prefix "custom prefix " FILENAME1 FILENAME2 FILENAME3
  email_sigs --erase-all  # erases signatures with default prefix 
  email_sigs --prefix "custom prefix " --erase-all
"#,
            default_prefix()
        );
        std::process::exit(exit_code::SUCCESS);
    }

    println!("Using prefix <{}>", &args.prefix);

    if args.erase_all {
        println!("Erasing signatures with prefix <{}>", &args.prefix);
        let num_erased = erase_all_signatures(&args.prefix)?;
        println!("{} erased", num_erased);
        std::process::exit(exit_code::SUCCESS);
    }

    if args.filenames.is_empty() {
        eprintln!("No filenames provided");
        std::process::exit(exit_code::USAGE_ERROR);
    }
    let mut all_quotes: IndexSet<quotes_parser::Quote> = IndexSet::new();

    for filename in args.filenames {
        let path = PathBuf::from(filename);
        if !path.is_file() {
            eprintln!("File <{}> cannot be read", path.display());
            std::process::exit(exit_code::IO_ERROR);
        }
        let contents = fs::read_to_string(path)?;
        if let Some(quotes) = quotes_parser::parse(&contents) {
            all_quotes.extend(quotes);
        }
    }

    println!("updating, number of quotes = {}", &all_quotes.len());
    erase_all_signatures(&args.prefix)?;
    add_signatures(
        &args.prefix,
        all_quotes
            .into_iter()
            .enumerate()
            .map(|(idx, quote)| {
                let key = format!("{}: {}", idx + 1, format_key(&quote));
                Signature::new(&key, &format_quote(&quote))
            })
            .collect(),
    )?;
    Ok(())
}
