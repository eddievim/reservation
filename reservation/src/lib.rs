mod error;

pub use error::ReservationError;

pub type ReservationID = String;

pub trait Rsvp {
    fn reverse(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    fn change_status(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError>;
    fn update_note(&self, id: ReservationID, note: String) -> Result<abi::Reservation, ReservationError>;
    fn delete(&self, id: ReservationID) -> Result<(), ReservationError>;
    fn get(&self, id: ReservationID) -> Result<abi::Reservation, ReservationError>;
    fn all_reversation(&self, query: abi::Query) -> Result<Vec<abi::Reservation>, ReservationError>;
}