#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use std::process;
use std::path::Path;
use inline_colorization::*;
fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let config = nu_hist::Config::new(&args).unwrap_or_else(|err| {
    eprintln!("{color_red}Problem parsing arguments: {}", 
    err
  );
    process::exit(1);
  });

  let path = Path::new(&config.path);
  if !path.exists() {
    eprintln!("{color_red}{style_bold}File does not exist: {style_reset}{color_reset}{}", config.path);
    process::exit(1);
  } else if check_table(path).is_err() {
    eprintln!("{color_red}{style_bold}Invalid file: {style_reset}{color_reset}{}", config.path);
    process::exit(1);

  } else if config.analysis == "all" {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::all(conn)?;
  } else if config.analysis.parse::<i32>().is_ok() {
    let conn: Connection = Connection::open(&config.path)?;
    nu_hist::year(conn, config.analysis)?;
  } else {
    eprintln!("{color_red}{style_bold}Invalid year: {style_reset}{color_reset}{}", config.analysis);
    process::exit(1);
  }
  return Ok(());
}

fn check_table(path: &Path) -> Result<()> {
  let conn = Connection::open(path)?;
  let mut content: rusqlite::Statement<'_> = conn.prepare(
    "SELECT name FROM sqlite_master WHERE type='table' AND name='history'"
  )?;
  let mut rows = content.query([])?;
  if let Some(row) = rows.next()? {
    let name: String = row.get(0)?;
    if name == "history" {
      return Ok(());
    }
  }
  eprintln!("{color_red}{style_bold}No history table found in file: {style_reset}{color_reset}{}", &path.display());
  process::exit(1);
}