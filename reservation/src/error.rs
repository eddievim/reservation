use thiserror::Error;

#[derive(Error, Debug)]

pub enum ReservationError {
    #[error("database error")]
    DbError(#[from] sqlx::Error),

    #[error("invalid start time or end time")]
    InvalidTime,
    #[error("unknown error")]
    Unknown,
}