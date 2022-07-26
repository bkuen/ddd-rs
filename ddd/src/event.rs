use std::any::Any;
use std::fmt::Debug;
use chrono::{DateTime, Utc};
use downcast_rs::{Downcast, impl_downcast};
use uuid::Uuid;

pub trait DomainEvent: Debug + Send + Sync + Downcast {
    fn id(&self) -> Uuid;
    fn timestamp(&self) -> DateTime<Utc>;
}

impl_downcast!(DomainEvent);