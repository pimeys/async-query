use crate::{dbio::DBIO, queryable::{Queryable, Sqlite, Postgres, Mysql}};
use tokio_resource_pool::{Manage, Status, RealDependencies, CheckOut};

pub struct MysqlManager;
pub struct PostgresManager;
pub struct SqliteManager;

impl Manage for PostgresManager {
    type Resource = Postgres;
    type Dependencies = RealDependencies;
    type CheckOut = CheckOut<Self>;
    type Error = crate::AnyError;
    type CreateFuture = DBIO<'static, Self::Resource>;
    type RecycleFuture = DBIO<'static, Option<Self::Resource>>;

    fn create(&self) -> Self::CreateFuture {
        Postgres::new()
    }

    fn status(&self, _: &Self::Resource) -> Status {
        Status::Valid
    }

    fn recycle(&self, connection: Self::Resource) -> Self::RecycleFuture {
        DBIO::new(async {
            connection.select_1().await?;
            Ok(Some(connection))
        })
    }
}

impl Manage for MysqlManager {
    type Resource = Mysql;
    type Dependencies = RealDependencies;
    type CheckOut = CheckOut<Self>;
    type Error = crate::AnyError;
    type CreateFuture = DBIO<'static, Self::Resource>;
    type RecycleFuture = DBIO<'static, Option<Self::Resource>>;

    fn create(&self) -> Self::CreateFuture {
        Mysql::new()
    }

    fn status(&self, _: &Self::Resource) -> Status {
        Status::Valid
    }

    fn recycle(&self, connection: Self::Resource) -> Self::RecycleFuture {
        DBIO::new(async {
            connection.select_1().await?;
            Ok(Some(connection))
        })
    }
}

impl Manage for SqliteManager {
    type Resource = Sqlite;
    type Dependencies = RealDependencies;
    type CheckOut = CheckOut<Self>;
    type Error = crate::AnyError;
    type CreateFuture = DBIO<'static, Self::Resource>;
    type RecycleFuture = DBIO<'static, Option<Self::Resource>>;

    fn create(&self) -> Self::CreateFuture {
        Sqlite::new()
    }

    fn status(&self, _: &Self::Resource) -> Status {
        Status::Valid
    }

    fn recycle(&self, connection: Self::Resource) -> Self::RecycleFuture {
        DBIO::new(async {
            connection.select_1().await?;
            Ok(Some(connection))
        })
    }
}
