
use crate::abi;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reverse(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        let sql = "INSERT INTO reservation (id, name, email, status, note) VALUES ($1, $2, $3, $4, $5) RETURNING ID";
        sqlx::Query::execute(sql,
         rsvp.id, rsvp.name, rsvp.email, rsvp.status, rsvp.note).await?;
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