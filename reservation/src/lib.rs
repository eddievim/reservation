mod error;

use async_trait::async_trait;
pub use error::ReservationError;
use sqlx::{postgres::Postgres, Pool};

pub type ReservationID = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: Pool::<Postgres>,
}

#[async_trait]
pub trait Rsvp {
    async fn reverse(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    async fn change_status(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError>;
    async fn update_note(&self, id: ReservationID, note: String) -> Result<abi::Reservation, ReservationError>;
    async fn delete(&self, id: ReservationID) -> Result<(), ReservationError>;
    async fn get(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError>;
    async fn all_reversation(&self, query: abi::Query) -> Result<Vec<abi::Reservation>, ReservationError>;
}