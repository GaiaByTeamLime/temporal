use ormx::Table;
use sqlx::PgPool;
use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Debug, Table, Serialize)]
#[serde(crate = "rocket::serde")]
#[ormx(table = "timeseries", id = snapshot_id, insertable, deletable)]
pub struct Snapshot {
    #[ormx(column = "id")]
    #[ormx(get_optional(i32))]
    #[ormx(default)]
    pub snapshot_id: i32,

    pub sensor_uid: String,

    #[ormx(default)]
    pub created: NaiveDateTime,

    pub illumination: f32,
    pub humidity: f32,
    pub temperature: f32,
    pub voltage: f32,
    pub soil_humidity: f32,
    pub soil_salt: f32,
}

pub async fn get_newest_snapshots(
    db: &PgPool,
    limit: Option<(usize, usize)>,
) -> Result<Vec<Snapshot>, sqlx::Error> {
    let result = ormx::conditional_query_as!(
        Snapshot,
        "SELECT DISTINCT ON (sensor_uid)
            id AS snapshot_id,
            sensor_uid,
            created,
            illumination,
            humidity,
            temperature,
            voltage,
            soil_humidity,
            soil_salt
        "
        "FROM timeseries"
        "ORDER BY sensor_uid, created DESC"
        Some((l, o)) = limit => {
            "LIMIT" ?(l as i64)
            "OFFSET" ?(o as i64)
        }
    )
    .fetch_all(db)
    .await?;

    Ok(result)
}