#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use rand::Rng;
use std::process;
use chrono::*;

pub struct Config {
  pub path: String,
  pub analysis: String,
}
impl Config {
  pub fn new(args: &[String]) -> Result<Config> {
    if args.len() < 3 {
      eprintln!(
        "Not enough arguments: nu-hist [path to history.sqlite3 file] [year as number | 'all']"
      );
      process::exit(1);
    }
    let path = args[1].clone();
    let analysis = args[2].clone();
    return Ok(Config { path, analysis });
  }
}

pub fn year(conn: Connection, year: String) -> Result<()> {
  let year: i32 = year.to_string().parse::<i32>().unwrap();
  let (start, end) = year_to_unix(year);

  let mut content: rusqlite::Statement<'_> = conn.prepare(
    "select * from history where start_timestamp >= ?1 AND start_timestamp <= ?2"
  )?;
  let mut rows = content.query([start, end])?;

  let mut arr: Vec<String> = Vec::new();
  while let Some(row) = rows.next()? {
    let command: String = row.get(1)?;
    arr.push(command);
  }

  let _ = top_ten_dur(&conn, start, end);

  println!("Year: {}", year);
  println!("Total commands: {}", arr.len());
  return Ok(());
}

pub fn top_ten_dur(conn: &Connection, start: i64, end: i64) -> Result<()> {
  let mut content: rusqlite::Statement<'_> = conn.prepare(
    "SELECT
    CASE WHEN instr(command_line, ' ') > 0
        THEN substr(command_line, 1, instr(command_line, ' ') - 1)
        ELSE command_line
    END AS first_word,
    CAST(count(*) AS VARCHAR) AS count
FROM history
WHERE start_timestamp >= ?1 AND start_timestamp <= ?2
GROUP BY first_word
ORDER BY CAST(count(*) AS INTEGER) DESC
LIMIT 10;"
  )?;
  let mut rows = content.query([start, end])?;
  let mut arr: Vec<(String, String)> = Vec::new();
  let mut i = 0;
  while let Some(row) = rows.next()? {
    let a: String = row.get(0)?;
    let b: String = row.get(1)?;
    println!("{} - {}: {}", i + 1, a, b);
    i = i + 1;
    arr.push((a, b));
  }
  return Ok(());
}

pub fn all(conn: Connection) -> Result<()> {
  let _ = top_ten_dur(&conn, 0, i64::MAX);
  let mut content: rusqlite::Statement<'_> = conn.prepare("SELECT COUNT(*) from history")?;
  let mut rows = content.query([])?;
  if let Some(row) = rows.next()? {
    let len: i64 = row.get(0)?;
    println!("Total commands: {}", len);
  }
  return Ok(());
}

fn year_to_unix(year: i32) -> (i64, i64) {
  let start = Utc.with_ymd_and_hms(year, 01, 01, 00, 00, 00).unwrap().timestamp();
  let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap().timestamp();
  return (start * 1000, end * 1000);
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
  return Ok(());
} // Haha this is just for side effects
