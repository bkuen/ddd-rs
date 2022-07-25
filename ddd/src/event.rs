use std::fmt::Debug;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait DomainEvent: Debug {
    fn id(&self) -> Uuid;
    fn timestamp(&self) -> DateTime<Utc>;
}