use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::Error;

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Score {
    pub rank: i64,
    pub id: i64,
    pub score: i64,
}

/// Global
pub async fn get_global_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    todo!();
    // let id = i64::try_from(id)?;
    // let row = sqlx::query_file_as!(Score, "src/data/score/sql/get_global_score_for_id.sql", id)
    //     .fetch_one(db)
    //     .await?;
    // Ok(row)
}

pub async fn show_global_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(Score, "src/data/score/sql/show_global_scoreboard.sql")
        .fetch_all(db)
        .await?;
    Ok(scoreboard)
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

pub async fn reset_dota_scoreboard(db: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query_file!("src/data/score/sql/dota/reset_dota_scoreboard.sql")
        .execute(db)
        .await?;
    Ok(())
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

pub async fn reset_cs_scoreboard(db: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query_file!("src/data/score/sql/cs/reset_cs_scoreboard.sql")
        .execute(db)
        .await?;
    Ok(())
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

pub async fn reset_rivals_scoreboard(db: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query_file!("src/data/score/sql/rivals/reset_rivals_scoreboard.sql")
        .execute(db)
        .await?;
    Ok(())
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

// MLBB
pub async fn get_mlbb_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    let id = i64::try_from(id)?;
    let row = sqlx::query_file_as!(
        Score,
        "src/data/score/sql/mlbb/get_mlbb_score_for_id.sql",
        id
    )
    .fetch_one(db)
    .await?;
    Ok(row)
}

pub async fn get_mlbb_winners(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let rows = sqlx::query_file_as!(Score, "src/data/score/sql/mlbb/get_mlbb_winners.sql")
        .fetch_all(db)
        .await?;
    Ok(rows)
}

pub async fn reset_mlbb_scoreboard(db: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query_file!("src/data/score/sql/mlbb/reset_mlbb_scoreboard.sql")
        .execute(db)
        .await?;
    Ok(())
}

pub async fn show_mlbb_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard =
        sqlx::query_file_as!(Score, "src/data/score/sql/mlbb/show_mlbb_scoreboard.sql")
            .fetch_all(db)
            .await?;
    Ok(scoreboard)
}

pub async fn update_mlbb_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/mlbb/update_mlbb_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

// HoK
pub async fn get_hok_score_for_id(db: &Pool<Sqlite>, id: u64) -> Result<Score, Error> {
    let id = i64::try_from(id)?;
    let row = sqlx::query_file_as!(Score, "src/data/score/sql/hok/get_hok_score_for_id.sql", id)
        .fetch_one(db)
        .await?;
    Ok(row)
}

pub async fn get_hok_winners(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let rows = sqlx::query_file_as!(Score, "src/data/score/sql/hok/get_hok_winners.sql")
        .fetch_all(db)
        .await?;
    Ok(rows)
}

pub async fn reset_hok_scoreboard(db: &Pool<Sqlite>) -> Result<(), Error> {
    sqlx::query_file!("src/data/score/sql/hok/reset_hok_scoreboard.sql")
        .execute(db)
        .await?;
    Ok(())
}

pub async fn show_hok_scoreboard(db: &Pool<Sqlite>) -> Result<Vec<Score>, Error> {
    let scoreboard = sqlx::query_file_as!(Score, "src/data/score/sql/hok/show_hok_scoreboard.sql")
        .fetch_all(db)
        .await?;
    Ok(scoreboard)
}

pub async fn update_hok_scoreboard(db: &Pool<Sqlite>, id: u64) -> Result<(), Error> {
    let id = i64::try_from(id)?;
    sqlx::query_file!("src/data/score/sql/hok/update_hok_scoreboard.sql", id)
        .execute(db)
        .await?;
    Ok(())
}
