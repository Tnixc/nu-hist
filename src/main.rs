#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use std::process;

// /Users/tnixc/Library/Application Support/nushell/history.sqlite3
fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let config = nusqliter::Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  let conn: Connection = Connection::open(&config.path)?;
  nusqliter::year(conn)?;
  return Ok(())
}
