#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
use rusqlite::{ Connection, Result };
use rand::Rng;

pub struct Config {
  pub path: String,
  pub analysis: String,
}
impl Config {
  pub fn new(args: &[String]) -> Result<Config> {
    if args.len() < 3 {
      panic!("Not enough arguments");
    }
    let path = args[1].clone();
    let analysis = args[2].clone();
    return Ok(Config { path, analysis })
  }
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
