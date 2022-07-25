use crate::event::DomainEvent;

pub trait AggregateRoot {
    fn events(&self) -> &Vec<DomainEvent>;

    fn add_event(&mut self, event: DomainEvent);
    fn clear_events(&mut self);
}