use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, Pool, Sqlite, Type};

use crate::Error;

#[allow(clippy::upper_case_acronyms)]
#[derive(Type, Debug, Copy, Clone, poise::ChoiceParameter)]
#[sqlx(type_name = "type")]
pub enum EventType {
    #[name = "Dota"]
    Dota,

    #[name = "CS"]
    CS,

    #[name = "Rivals"]
    Rivals,

    #[name = "MLBB"]
    MLBB,

    #[name = "HoK"]
    HoK,

    #[name = "Other"]
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

    pub async fn delete(db: &Pool<Sqlite>, name: &str, time: i64) -> Result<(), Error> {
        sqlx::query_file!("src/data/event/sql/delete_event.sql", name, time)
            .execute(db)
            .await?;
        Ok(())
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

    // Dota
    pub async fn get_dota_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let dota_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/dota/get_dota_events.sql",
            start,
            end
        )
        .fetch_all(db)
        .await?;

        Ok(dota_rows)
    }

    pub async fn get_dota_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let dota_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/dota/get_dota_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;

        Ok(dota_rows)
    }

    // CS
    pub async fn get_cs_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let cs_rows =
            sqlx::query_file_as!(Event, "src/data/event/sql/cs/get_cs_events.sql", start, end)
                .fetch_all(db)
                .await?;
        Ok(cs_rows)
    }

    pub async fn get_cs_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let cs_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/cs/get_cs_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(cs_rows)
    }

    // Rivals
    pub async fn get_rivals_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let rivals_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/rivals/get_rivals_events.sql",
            start,
            end
        )
        .fetch_all(db)
        .await?;
        Ok(rivals_rows)
    }

    pub async fn get_rivals_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let rivals_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/rivals/get_rivals_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(rivals_rows)
    }

    // MLBB
    pub async fn get_mlbb_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let mlbb_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/mlbb/get_mlbb_events.sql",
            start,
            end
        )
        .fetch_all(db)
        .await?;
        Ok(mlbb_rows)
    }

    pub async fn get_mlbb_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let mlbb_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/mlbb/get_mlbb_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(mlbb_rows)
    }

    // HoK
    pub async fn get_hok_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let hok_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/hok/get_hok_events.sql",
            start,
            end
        )
        .fetch_all(db)
        .await?;
        Ok(hok_rows)
    }

    pub async fn get_hok_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let hok_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/hok/get_hok_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(hok_rows)
    }

    // Others
    pub async fn get_other_events(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
    ) -> Result<Vec<Event>, Error> {
        let other_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/other/get_other_events.sql",
            start,
            end
        )
        .fetch_all(db)
        .await?;
        Ok(other_rows)
    }

    pub async fn get_other_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let other_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/other/get_other_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(other_rows)
    }

    pub async fn get_events_for_id(
        db: &Pool<Sqlite>,
        start: i64,
        end: i64,
        gardener_id: i64,
    ) -> Result<Vec<Event>, Error> {
        let event_rows = sqlx::query_file_as!(
            Event,
            "src/data/event/sql/get_events_for_id.sql",
            start,
            end,
            gardener_id
        )
        .fetch_all(db)
        .await?;
        Ok(event_rows)
    }

    pub fn gardener_name(&self) -> String {
        match self.gardeners {
            293_360_731_867_316_225 => String::from("Nik"),
            204_923_365_205_475_329 => String::from("Kit"),
            754_724_309_276_164_159 => String::from("WW"),
            172_360_818_715_918_337 => String::from("Bonteng"),
            332_438_787_588_227_072 => String::from("Sam"),
            _ => panic!("Something unexpected has happened"),
        }
    }
}
