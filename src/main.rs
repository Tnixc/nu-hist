use rusqlite::{ Connection, Result };

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
  
  Ok(())
}
