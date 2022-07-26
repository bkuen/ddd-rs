//! This library provides building blocks for domain driven design.
//! At the moment, it focuses on aggregate roots and events.
//!
//! # Usage
//!
//! ```rust
//! use std::sync::Arc;
//!
//! use chrono::{DateTime, Utc};
//! use ddd_rs::prelude::*;
//! use uuid::Uuid;
//!
//! #[derive(Debug, AggregateRoot)]
//! pub struct Cart {
//!     pub cart_id: Uuid,
//!     pub events: Vec<Box<dyn DomainEvent>>,
//! }
//!
//! #[derive(Debug, DomainEvent)]
//! pub struct CartCreated {
//!     pub id: Uuid,
//!     pub timestamp: DateTime<Utc>,
//! }
//!
//! ```

pub mod aggregate_root;
pub mod core;
pub mod event;

pub mod prelude {
    pub use crate::aggregate_root::AggregateRoot;
    pub use crate::event::DomainEvent;

    extern crate ddd_codegen_rs;
    pub use ddd_codegen_rs::AggregateRoot;
    pub use ddd_codegen_rs::DomainEvent;
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::prelude::*;

    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use test_context::{test_context, TestContext};
    use uuid::Uuid;

    #[derive(Debug, AggregateRoot)]
    struct TestAggregateRoot {
        aggregate_id: Uuid,
        events: Vec<Box<dyn DomainEvent>>,
    }

    #[derive(Debug, DomainEvent)]
    struct TestEvent {
        id: Uuid,
        timestamp: DateTime<Utc>,
    }

    impl Default for TestEvent {
        fn default() -> Self {
            Self {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
            }
        }
    }

    struct Context {
        aggregate_root: TestAggregateRoot,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            Self {
                aggregate_root: TestAggregateRoot {
                    aggregate_id: Uuid::new_v4(),
                    events: vec![],
                }
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_aggregate_root_aggregate_id(ctx: &mut Context) {
        assert!(!ctx.aggregate_root.aggregate_id.is_nil())
    }

    #[test_context(Context)]
    #[test]
    fn test_aggregate_root_clear_events(ctx: &mut Context) {
        ctx.aggregate_root.clear_events();
        assert!(ctx.aggregate_root.events.is_empty())
    }

    #[test_context(Context)]
    #[test]
    fn test_aggregate_root_add_event(ctx: &mut Context) {
        let len = ctx.aggregate_root.events.len();
        let event = Box::new(TestEvent::default());

        ctx.aggregate_root.add_event(event);
        assert_eq!(ctx.aggregate_root.events.len(), len + 1);
    }

    #[test]
    fn test_downcast() {
        let event = Box::new(TestEvent::default());
        let events: Vec<Box<dyn DomainEvent >> = vec![event];

        for event in events.iter() {
            if let Some(abc) = event.downcast_ref::<TestEvent>() {
                continue;
            }

            panic!("event could not be downcasted");
        }
    }

}