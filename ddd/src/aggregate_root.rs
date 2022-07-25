use std::fmt::Debug;
use crate::event::DomainEvent;

use std::sync::Arc;
use uuid::Uuid;

pub trait AggregateRoot: Debug {
    fn aggregate_id(&self) -> Uuid;
    fn events(&self) -> &Vec<Arc<dyn DomainEvent>>;

    fn add_event(&mut self, event: Arc<dyn DomainEvent>);
    fn clear_events(&mut self);
}