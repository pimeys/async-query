use super::DBIO;
use futures::future::FutureExt;
use async_std::sync::Mutex;
use tokio_postgres::{NoTls, Config};

pub trait Queryable {
    fn select_1(&self) -> DBIO<u32>;
}

pub struct Sqlite {
    client: Mutex<rusqlite::Connection>,
}

impl Sqlite {
    pub fn new() -> crate::Result<Self> {
        Ok(Self { client: Mutex::new(rusqlite::Connection::open_in_memory()?) })
    }
}

impl Queryable for Sqlite {
    fn select_1(&self) -> DBIO<u32> {
        DBIO::new(async move {
            let client = self.client.lock().await;
            let mut stmt = client.prepare_cached("SELECT 1")?;
            let mut rows = stmt.query(rusqlite::NO_PARAMS)?;
            let row = rows.next()?.unwrap();

            Ok(row.get(0)?)
        })
    }
}

pub struct Postgres {
    client: Mutex<tokio_postgres::Client>
}

impl Postgres {
    pub async fn new() -> crate::Result<Self> {
        let (client, conn) = Config::new()
            .user("postgres")
            .password("prisma")
            .host("localhost")
            .connect(NoTls)
            .await?;

        let connection = conn.map(|r| r.unwrap());

        tokio::spawn(connection);

        Ok(Self { client: Mutex::new(client) })
    }
}

impl Queryable for Postgres {
    fn select_1(&self) -> DBIO<u32> {
        DBIO::new(async move {
            let client = self.client.lock().await;
            let stmt = client.prepare("SELECT 1").await?;
            let rows = client.query(&stmt, &[]).await?;

            let row = rows.first().unwrap();
            let result: i32 = row.get(0);

            Ok(result as u32)
        })
    }
}
