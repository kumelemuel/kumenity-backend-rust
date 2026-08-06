use chrono::{DateTime, Utc};
use shared::domain::domain_event::DomainEvent;

#[derive(Debug, Clone)]
pub enum AccountEvent {
    Registered,
}
impl DomainEvent for AccountEvent {
    fn event_type(&self) -> &'static str {
        todo!()
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        todo!()
    }

    fn aggregate_id(&self) -> String {
        todo!()
    }
}
