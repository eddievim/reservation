mod error;

pub use error::ReservationError;

pub type ReservationID = String;

pub trait Rsvp {
    fn reverse(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    fn change_status(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError>;
}