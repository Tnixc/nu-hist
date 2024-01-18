use rusqlite::{ Connection, Result };
use rand::Rng;

fn main() -> Result<()> {
  // Connect to the database (create it if it doesn't exist)
  let conn = Connection::open("history.sqlite3")?;
  // let mut content = conn.prepare("select * from history")?;
  // let mut rows = content.query([])?;
  // while let Some(row) = rows.next()? {
  //   let time: i64 = row.get(2)?;
  //   let command: String = row.get(1)?;
  //   println!("{}: {:?}", time, command)
  // }
  let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  for _ in 0..100 {
    let command = rand_string(3, chars);
    let hostname = rand_string(4, chars);
    let time: String = rand
      ::thread_rng()
      .gen_range(0..1000000000)
      .to_string();
    conn.execute(
      "insert into history (command_line, time, hostname) values (?1, ?2, ?3)",
      &[&command, &time, &hostname]
    )?;
  }
  Ok(())
}

fn rand_string(len: usize, chars: &str) -> String {
  let mut rng = rand::thread_rng();
  let mut s = String::with_capacity(len);
  for _ in 0..len {
    let idx = rng.gen_range(0..chars.len());
    s.push(chars.chars().nth(idx).unwrap());
  }
  return s;
}
