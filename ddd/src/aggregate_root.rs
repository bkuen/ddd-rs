use uuid::Uuid;
use crate::event::DomainEvent;

pub trait AggregateRoot {
    fn aggregate_id(&self) -> Uuid;
    fn events(&self) -> &Vec<Box<dyn DomainEvent>>;

    fn add_event(&mut self, event: Box<dyn DomainEvent>);
    fn clear_events(&mut self);
}