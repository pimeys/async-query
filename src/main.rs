mod dbio;
mod queryable;

use dbio::DBIO;
use queryable::{Queryable, Sqlite};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    let sqlite = Sqlite::new()?;
    println!("Number is {}", sqlite.select_1().await?);

    Ok(())
}
