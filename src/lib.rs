#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use rand::Rng;
use std::process;
pub struct Config {
  pub path: String,
  pub analysis: String,
}
impl Config {
  pub fn new(args: &[String]) -> Result<Config> {
    if args.len() < 3 {
      eprintln!("Arguents: nu-hist [path to history.sqlite3 file] [year as number | 'all']");
      process::exit(1);
    }
    let path = args[1].clone();
    let analysis = args[2].clone();
    return Ok(Config { path, analysis })
  }
}
pub fn year(conn: Connection, year: String) -> Result<()> {
  let mut content: rusqlite::Statement<'_> = conn.prepare("select * from history")?;
  let mut rows = content.query([])?;
  while let Some(row) = rows.next()? {
    let time: i64 = row.get(2)?;
    let command: String = row.get(1)?;
    // println!("{}: {:?}", time, command);
  }
  println!("Year: {}", year);
  return Ok(())
}

pub fn all(conn: Connection) -> Result<()> {
  let mut content: rusqlite::Statement<'_> = conn.prepare("select * from history")?;
  let mut rows = content.query([])?;
  while let Some(row) = rows.next()? {
    let time: i64 = row.get(2)?;
    let command: String = row.get(1)?;
    println!("{}: {:?}", time, command);
  }
  return Ok(())
}


pub fn rand_string(len: usize, chars: &str) -> String {
  let mut rng = rand::thread_rng();
  let mut s = String::with_capacity(len);
  for _ in 0..len {
    let idx = rng.gen_range(0..chars.len());
    s.push(chars.chars().nth(idx).unwrap());
  }
  return s;
}

pub fn fill_data(conn: &Connection) -> Result<()> {
  let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  for _ in 0..100_000 {
    let command = rand_string(3, chars);
    let hostname = rand_string(4, chars);
    let time: String = rand
      ::thread_rng()
      .gen_range(1671512400..1704949200)
      .to_string();
    conn.execute(
      "insert into history (command_line, start_timestamp, hostname) values (?1, ?2, ?3)",
      [&command, &time, &hostname]
    )?;
  }
  return Ok(())
} // Haha this is just for side effects

