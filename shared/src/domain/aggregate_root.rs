use crate::domain::{domain_event::DomainEvent, entity::Entity};

pub trait AggregateRoot: Entity {
    type Event: DomainEvent;

    fn record_event(&mut self, event: Self::Event);
    fn pull_events(&mut self) -> Vec<Self::Event>;
    fn version(&self) -> u64;
}

#[derive(Clone, Debug)]
pub struct EventRecorder<E> {
    pending: Vec<E>,
}

impl<E> Default for EventRecorder<E> {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
        }
    }
}

impl<E> EventRecorder<E> {
    pub fn record(&mut self, event: E) {
        self.pending.push(event);
    }

    pub fn pull(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending)
    }
}
