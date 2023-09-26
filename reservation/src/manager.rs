use std::time::SystemTime;

use async_trait::async_trait;
use sqlx::{postgres::types::PgRange, Row};
use chrono::{Utc, DateTime};

use crate::{Rsvp, ReservationManager, ReservationID, ReservationError};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reverse(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        let start = rsvp.start.unwrap();
        let start = DateTime::<Utc>::from_timestamp(
            start.seconds,
            start.nanos as u32).unwrap();
        let end = rsvp.end.unwrap();
        let end = DateTime::<Utc>::from_timestamp(
            end.seconds,
            end.nanos as u32).unwrap();
        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        let status: abi::ReservationStatus = abi::ReservationStatus::try_from(rsvp.status)
        .unwrap_or(abi::ReservationStatus::Pending);
    
         let id = sqlx::query("INSERT INTO reservation(user_id, resource_id, timespan, note, status)
         VALUES ($1, $2, $3, $4, $5) RETURNING ID")
         .bind(rsvp.user_id)
         .bind(rsvp.resource_id)
         .bind(timespan)
         .bind(rsvp.note)
         .bind(status)
         .fetch_one(&self.pool)
         .await?.get(0);

        rsvp.id = id;
    
        Ok(rsvp)
    }

    async fn change_status(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(&self, id: ReservationID, note: String) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, id: ReservationID) -> Result<(), ReservationError> {
        todo!()
    }

    async fn get(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn all_reversation(&self, query: abi::Query) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}
