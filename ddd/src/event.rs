use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait DomainEvent {
    fn id(&self) -> Uuid;
    fn timestamp(&self) -> DateTime<Utc>;
}