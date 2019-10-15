mod dbio;
mod queryable;

use dbio::DBIO;
use queryable::{Queryable, Sqlite, Postgres, Mysql};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    let sqlite = Sqlite::new()?;
    let psql = Postgres::new().await?;
    let mysql = Mysql::new()?;

    println!("Number from Sqlite: {}", sqlite.select_1().await?);
    println!("Number from Postgres: {}", psql.select_1().await?);
    println!("Number from Mysql: {}", mysql.select_1().await?);

    Ok(())
}
