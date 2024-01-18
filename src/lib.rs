#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
// use rand::Rng;
use std::process;
use chrono::*;
use inline_colorization::*;
use comfy_table::*;

pub struct Config {
  pub path: String,
  pub analysis: String,
}
impl Config {
  pub fn new(args: &[String]) -> Result<Config> {
    if args.len() < 3 {
      eprintln!(
        "{color_red}{style_bold}Not enough arguments: {style_reset}{color_reset}nu-hist [path to history.sqlite3 file] [year as number | 'all']"
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
  let mut head = Table::new();
  head.set_header(
    vec![
      Cell::new("Year: ".to_string() + &year.to_string())
        .add_attribute(Attribute::Bold)
        .fg(Color::Magenta),
      Cell::new("Total Commands: ".to_string() + &arr.len().to_string())
        .add_attribute(Attribute::Bold)
        .fg(Color::Magenta)
    ]
  );
  println!("{}", head);
  println!("{}", table(top_ten_dur(&conn, start, end).unwrap(), arr.len() as i64));
  return Ok(());
}

pub fn all(conn: Connection) -> Result<()> {
  let mut content: rusqlite::Statement<'_> = conn.prepare("SELECT COUNT(*) from history")?;
  let mut rows = content.query([])?;
  if let Some(row) = rows.next()? {
    let len: i64 = row.get(0)?;
    let mut head = Table::new();
    head.set_header(
      vec![
        Cell::new("Total Commands: ".to_string() + &len.to_string())
          .add_attribute(Attribute::Bold)
          .fg(Color::Magenta)
      ]
    );
    println!("{}", head);
    println!("{}", table(top_ten_dur(&conn, 0, i64::MAX).unwrap(), len));
  }
  return Ok(());
}

fn top_ten_dur(conn: &Connection, start: i64, end: i64) -> Result<Vec<(String, String)>> {
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
  while let Some(row) = rows.next()? {
    let a: String = row.get(0)?;
    let b: String = row.get(1)?;
    arr.push((a, b));
  }
  return Ok(arr);
}
fn table(arr: Vec<(String, String)>, total: i64) -> Table {
  let mut table = Table::new();
  table.set_header(
    vec![
      Cell::new("#").add_attribute(Attribute::Bold).fg(Color::Green),
      Cell::new("Command").add_attribute(Attribute::Bold).fg(Color::Green),
      Cell::new("Count").add_attribute(Attribute::Bold).fg(Color::Green),
      Cell::new("Bar").add_attribute(Attribute::Bold).fg(Color::Green)
    ]
  );
  let mut i = 0;
  for (a, b) in arr {
    let mut x: String = String::new();
    for _ in 0..(b.parse::<i64>().unwrap() * 100) / total {
      x = x + "■";
    }
    table.add_row(
      vec![
        Cell::new(&i.to_string()).fg(color_by_index(i % 6)),
        Cell::new(&a).fg(color_by_index(i % 6)),
        Cell::new(&b).fg(color_by_index(i % 6)),
        Cell::new(&x).fg(color_by_index(i % 6))
      ]
    );
    i = i + 1;
  }
  return table;
}
fn color_by_index(index: usize) -> Color {
  match index {
    0 => Color::Red,
    1 => Color::Green,
    2 => Color::Yellow,
    3 => Color::Blue,
    4 => Color::Magenta,
    5 => Color::Cyan,
    6 => Color::White,
    _ => Color::Reset,
  }
}

fn year_to_unix(year: i32) -> (i64, i64) {
  let start = Utc.with_ymd_and_hms(year, 1, 1, 00, 00, 00).unwrap().timestamp();
  let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap().timestamp();
  return (start * 1000, end * 1000);
}

// pub fn rand_string(len: usize, chars: &str) -> String {
//   let mut rng = rand::thread_rng();
//   let mut s = String::with_capacity(len);
//   for _ in 0..len {
//     let idx = rng.gen_range(0..chars.len());
//     s.push(chars.chars().nth(idx).unwrap());
//   }
//   return s;
// }

// pub fn fill_data(conn: &Connection) -> Result<()> {
//   let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
//   for _ in 0..100_000 {
//     let command = rand_string(3, chars);
//     let hostname = rand_string(4, chars);
//     let time: String = rand
//       ::thread_rng()
//       .gen_range(1671512400..1704949200)
//       .to_string();
//     conn.execute(
//       "insert into history (command_line, start_timestamp, hostname) values (?1, ?2, ?3)",
//       [&command, &time, &hostname]
//     )?;
//   }
//   return Ok(());
// } // Haha this is just for side effects
