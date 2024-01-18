#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use std::{process, num::ParseIntError};

// /Users/tnixc/Library/Application Support/nushell/history.sqlite3
fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let config = nu_hist::Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  let conn: Connection = Connection::open(&config.path)?;
  if config.analysis == "all" {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::all(conn)?;
  } else if let Ok(year) = config.analysis.parse::<i32>() {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::year(conn, config.analysis)?;
  } else {
    eprintln!("Invalid year: {}", config.analysis);
    process::exit(1);
  }
  return Ok(())
}
