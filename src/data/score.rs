use sqlx::{Pool, Sqlite, prelude::FromRow};

use crate::Error;

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Score {
    pub rank: i64,
    pub id: i64,
    pub score: i64,
}

pub async fn show_dota_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(Score, "src/data/score/sql/show_dota_scoreboard.sql")
        .fetch_all(db)
        .await?;

    Ok(scoreboard)
}

pub async fn update_dota_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/update_dota_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn show_cs_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(Score, "src/data/score/sql/show_cs_scoreboard.sql")
        .fetch_all(db)
        .await?;
    Ok(scoreboard)
}

pub async fn update_cs_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/update_cs_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}
