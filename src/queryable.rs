use super::DBIO;
use futures::future;
use async_std::sync::Mutex;

pub trait Queryable {
    fn select_1(&self) -> DBIO<'_, u32>;
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
    fn select_1(&self) -> DBIO<'_, u32> {
        DBIO::new(async move {
            let client = self.client.lock().await;
            let mut stmt = client.prepare_cached("SELECT 1")?;
            let mut rows = stmt.query(rusqlite::NO_PARAMS)?;
            let row = rows.next()?.unwrap();

            Ok(row.get(0)?)
        })
    }
}
