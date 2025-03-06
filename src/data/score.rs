use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::Error;

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Score {
    pub rank: i64,
    pub id: i64,
    pub score: i64,
}

// Dota
pub async fn get_dota_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    let id = i64::try_from(id)?;
    let row = sqlx::query_file_as!(
        Score,
        "src/data/score/sql/dota/get_dota_score_for_id.sql",
        id
    )
    .fetch_one(db)
    .await?;
    Ok(row)
}

pub async fn get_dota_winners(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let rows = sqlx::query_file_as!(Score, "src/data/score/sql/dota/get_dota_winners.sql")
        .fetch_all(db)
        .await?;
    Ok(rows)
}

pub async fn show_dota_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard =
        sqlx::query_file_as!(Score, "src/data/score/sql/dota/show_dota_scoreboard.sql")
            .fetch_all(db)
            .await?;

    Ok(scoreboard)
}

pub async fn update_dota_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/dota/update_dota_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

// CS
pub async fn get_cs_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    let id = i64::try_from(id)?;
    let row = sqlx::query_file_as!(Score, "src/data/score/sql/cs/get_cs_score_for_id.sql", id)
        .fetch_one(db)
        .await?;
    Ok(row)
}

pub async fn get_cs_winners(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let rows = sqlx::query_file_as!(Score, "src/data/score/sql/cs/get_cs_winners.sql")
        .fetch_all(db)
        .await?;
    Ok(rows)
}

pub async fn show_cs_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(Score, "src/data/score/sql/cs/show_cs_scoreboard.sql")
        .fetch_all(db)
        .await?;
    Ok(scoreboard)
}

pub async fn update_cs_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/cs/update_cs_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

// Rivals
pub async fn get_rivals_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    let id = i64::try_from(id)?;
    let row = sqlx::query_file_as!(
        Score,
        "src/data/score/sql/rivals/get_rivals_score_for_id.sql",
        id
    )
    .fetch_one(db)
    .await?;
    Ok(row)
}

pub async fn get_rivals_winners(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let rows = sqlx::query_file_as!(Score, "src/data/score/sql/rivals/get_rivals_winners.sql")
        .fetch_all(db)
        .await?;
    Ok(rows)
}

pub async fn show_rivals_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(
        Score,
        "src/data/score/sql/rivals/show_rivals_scoreboard.sql"
    )
    .fetch_all(db)
    .await?;
    Ok(scoreboard)
}

pub async fn update_rivals_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/rivals/update_rivals_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}
