use super::DBIO;
use futures::future;

pub trait Queryable {
    fn select_1(&self) -> DBIO<u32>;
}

pub struct Sqlite {
    client: rusqlite::Connection,
}

impl Sqlite {
    pub fn new() -> crate::Result<Self> {
        Ok(Self { client: rusqlite::Connection::open_in_memory()? })
    }
}

impl Queryable for Sqlite {
    fn select_1(&self) -> DBIO<u32> {
        let sync = || {
            let mut stmt = self.client.prepare_cached("SELECT 1")?;
            let mut rows = stmt.query(rusqlite::NO_PARAMS)?;
            let row = rows.next()?.unwrap();

            Ok(row.get(0)?)
        };

        match sync() {
            Ok(num) => DBIO::new(future::ok(num)),
            Err(e) => DBIO::new(future::err(e))
        }
    }
}
