use chrono::{DateTime, Utc};

pub trait DomainEvent: Sized + Clone {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn aggregate_id(&self) -> String;
    fn event_version(&self) -> u32 {
        1
    }
}
