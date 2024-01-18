use rusqlite::{ Connection, Result };

fn main() -> Result<()> {
  // Connect to the database (create it if it doesn't exist)
  let conn = Connection::open("history.sqlite3")?;
  conn.execute(
    "delete from history where id = 366;",
    []
  )?;
  Ok(())
}
