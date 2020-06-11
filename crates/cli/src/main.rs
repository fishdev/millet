//! A CLI for millet.

mod args;
mod diagnostic;
mod source;

use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use millet_core::{error, lex, parse};
use std::io::Write as _;

fn run() -> bool {
  let args = args::get();
  let config = term::Config::default();
  let writer = StandardStream::stdout(ColorChoice::Auto);
  let mut writer = writer.lock();
  let mut source_map = source::SourceMap::new();
  for name in args.files {
    match std::fs::read_to_string(&name) {
      Ok(s) => source_map.insert(name, s),
      Err(e) => {
        writeln!(writer, "io error: {}: {}", name, e).unwrap();
        return false;
      }
    }
  }
  for (id, file) in source_map.iter() {
    let lexer = match lex::get(file.as_bytes()) {
      Ok(x) => x,
      Err(e) => {
        term::emit(
          &mut writer,
          &config,
          &source_map,
          &diagnostic::new(id, e.loc.wrap(error::Error::Lex(e.val))),
        )
        .unwrap();
        return false;
      }
    };
    match parse::get(lexer) {
      Ok(xs) => eprintln!("parsed: {:#?}", xs),
      Err(e) => {
        term::emit(
          &mut writer,
          &config,
          &source_map,
          &diagnostic::new(id, e.loc.wrap(error::Error::Parse(e.val))),
        )
        .unwrap();
        return false;
      }
    }
  }
  true
}

fn main() {
  if !run() {
    std::process::exit(1);
  }
  println!("OK");
}
