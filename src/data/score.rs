use sqlx::{Pool, Sqlite};

use crate::Error;

#[allow(dead_code)]
pub struct Score {
    pub id: i64,
    pub point: i64,
}

pub async fn update_dota_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file_as!(Score, "src/data/score/sql/update_dota_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn update_cs_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file_as!(Score, "src/data/score/sql/update_cs_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

impl Score {}
