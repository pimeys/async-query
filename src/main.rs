mod dbio;
mod queryable;
mod pool;

use dbio::DBIO;
use queryable::Queryable;
use tokio_resource_pool::Builder;
use pool::{PostgresManager, MysqlManager, SqliteManager};

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, AnyError>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    {
        let pool = Builder::new().build(4, PostgresManager);
        let conn = pool.check_out().await?;

        println!("Number from Postgres: {}", conn.select_1().await?);
    }

    {
        let pool = Builder::new().build(4, MysqlManager);
        let conn = pool.check_out().await?;

        println!("Number from Mysql: {}", conn.select_1().await?);
    }

    {
        let pool = Builder::new().build(4, SqliteManager);
        let conn = pool.check_out().await?;

        println!("Number from Mysql: {}", conn.select_1().await?);
    }

    Ok(())
}
