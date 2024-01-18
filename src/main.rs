#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };


// /Users/tnixc/Library/Application Support/nushell/history.sqlite3
fn main() -> Result<()> {
  let conn = Connection::open("history.sqlite3")?;
  let mut content: rusqlite::Statement<'_> = conn.prepare("select * from history")?;
  let mut rows = content.query([])?;
  while let Some(row) = rows.next()? {
    let time: i64 = row.get(2)?;
    let command: String = row.get(1)?;
    println!("{}: {:?}", time, command);
  }
  return Ok(())
}
