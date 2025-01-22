#![allow(clippy::all)]

use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, Pool, Sqlite, Type};

use crate::Error;

#[derive(Type, Debug, Copy, Clone)]
#[sqlx(type_name = "type")]
pub enum EventType {
    Dota,
    CS,
    Other,
}

#[derive(Debug, FromRow)]
pub struct Event {
    pub name: String,
    pub time: i64,
    pub event_type: EventType,
    pub gardeners: i64,
    pub hours: i64,
}

impl Event {
    pub fn new(
        name: &str,
        time: NaiveDateTime,
        event_type: EventType,
        gardeners: i64,
        hours: i64,
    ) -> Self {
        Self {
            name: name.to_owned(),
            time: time.and_utc().timestamp(),
            event_type,
            gardeners,
            hours,
        }
    }

    pub async fn insert(&self, db: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query_file_as!(
            Event,
            "src/data/event/sql/insert_event.sql",
            self.name,
            self.time,
            self.event_type,
            self.gardeners,
            self.hours
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_dota_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let dota_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/get_dota_events.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;

        Ok(dota_rows)
    }

    pub async fn get_cs_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let cs_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/get_cs_events.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(cs_rows)
    }

    pub async fn get_other_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let other_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/get_other_events.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(other_rows)
    }
}
