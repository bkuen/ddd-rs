use crate::event::DomainEvent;

use std::fmt::Debug;
use std::sync::Arc;

use uuid::Uuid;

pub trait AggregateRoot: Debug {
    fn aggregate_id(&self) -> Uuid;
    fn events(&self) -> &Vec<Box<dyn DomainEvent>>;

    fn add_event(&mut self, event: Box<dyn DomainEvent>);
    fn clear_events(&mut self);
}