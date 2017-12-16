use std::io;

use super::Event;

pub trait EventPolling {
    fn poll(&mut self, events: &mut Vec<Event>) -> io::Result<()>;

    fn and_then<B: EventPolling>(self, other: B) -> CompositeEventPolling<Self, B>
    where Self: Sized {
        CompositeEventPolling{a: self, b: other}
    }
}

pub struct CompositeEventPolling<A: EventPolling, B: EventPolling> {
    a: A,
    b: B,
}

impl<A: EventPolling, B: EventPolling> EventPolling for CompositeEventPolling<A, B> {
    fn poll(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
        self.a.poll(events)?;
        self.b.poll(events)
    }
}

#[cfg(test)]
mod test {
    use super::super::{Event, Id, Value};
    use super::*;

    #[test]
    fn test_composite_event_polling() {
        let mut a = MockEventPolling::new();
        let mut b = MockEventPolling::new();

        a.events.push(Event{id: Id(1), value: Value(42)});
        a.events.push(Event{id: Id(2), value: Value(43)});
        b.events.push(Event{id: Id(3), value: Value(44)});
        b.events.push(Event{id: Id(4), value: Value(45)});

        let mut c = a.and_then(b);

        let mut events = Vec::new();
        let result = c.poll(&mut events);

        assert!(result.is_ok());
        assert_eq!(4, events.len());
        assert_eq!(Event{id: Id(1), value: Value(42)}, events[0]);
        assert_eq!(Event{id: Id(2), value: Value(43)}, events[1]);
        assert_eq!(Event{id: Id(3), value: Value(44)}, events[2]);
        assert_eq!(Event{id: Id(4), value: Value(45)}, events[3]);
    }

    struct MockEventPolling {
        pub events: Vec<Event>,
    }

    impl MockEventPolling {
        fn new() -> MockEventPolling {
            MockEventPolling {
                events: Vec::with_capacity(256),
            }
        }
    }

    impl EventPolling for MockEventPolling {
        fn poll(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
            for ev in self.events.iter() {
                events.push(*ev);
            }
            self.events.clear();
            Ok({})
        }
    }
}