use sqlx::PgPool;
use snapshot::{InsertSnapshot, Snapshot, get_newest_snapshots};
use ormx::Insert;

pub mod snapshot;

pub struct Timeseries {
    database: PgPool,
}

pub enum Error {
    Sql(sqlx::Error),
    Verify(String),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Sql(error)
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Sql(e) => e.to_string(),
            Self::Verify(e) => e.to_string(),
        }
    }
}

impl Timeseries {
    pub async fn new(connection: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(connection).await?;

        Ok(Self {
            database: db,
        })
    }

    pub async fn add(&self, item: InsertSnapshot) -> Result<i32, Error> {
        let mut db = self.database.acquire().await?;
        let inserted = item.insert(&mut db).await?;
        Ok(inserted.snapshot_id)
    }

    pub async fn get_all_last(&self) -> Result<Vec<Snapshot>, Error> {
        Ok(get_newest_snapshots(&self.database, None).await?)
    }
}