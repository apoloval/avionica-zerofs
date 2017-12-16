use std::io;
use std::thread;
use std::time;

use channel::PubChannel;
use event::Event;
use utils::Signal;

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

pub struct EventPublisher<P: EventPolling, C: PubChannel> {
    polling: P,
    channel: C,
    event_buffer: Vec<Event>,
    bytes_buffer: Vec<u8>,
}

impl<P: EventPolling, C: PubChannel> EventPublisher<P, C> {
    fn new(polling: P, channel: C) -> EventPublisher<P, C> {
        let event_buffer = Vec::with_capacity(1024);
        let bytes_buffer = Vec::with_capacity(64);
        EventPublisher{polling, channel, event_buffer, bytes_buffer }
    }

    fn run(&mut self, stop: Signal) {
        loop {
            // Terminate if stop is signalled
            if stop.status() { break; }

            // Poll pending events
            self.event_buffer.clear();
            if let Err(error) = self.polling.poll(&mut self.event_buffer) {
                error!("Failed to poll events: {}", error);
                continue;
            }

            // Send the polled events
            for event in self.event_buffer.iter() {
                self.bytes_buffer.clear();
                if let Err(error) = event.encode(&mut self.bytes_buffer) {
                    error!("Failed to encode event: {}", error);
                    continue
                }
                if let Err(error) = self.channel.write(&self.bytes_buffer) {
                    error!("Failed to write to pub channel: {}", error);
                }
            }
            
            // Sleep for a while
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use std::sync::mpsc;
    use std::thread;

    use channel::PubChannel;
    use event::{Event, Id, Value};
    use utils::Signal;
    use super::*;

    #[test]
    fn test_composite_event_polling() {
        let (a, txa) = MockEventPolling::new();
        let (b, txb) = MockEventPolling::new();
        let mut c = a.and_then(b);

        txa.send(Event{id: Id(1), value: Value(42)}).unwrap();
        txa.send(Event{id: Id(2), value: Value(43)}).unwrap();
        txb.send(Event{id: Id(3), value: Value(44)}).unwrap();
        txb.send(Event{id: Id(4), value: Value(45)}).unwrap();

        let mut events = Vec::new();
        let result = c.poll(&mut events);

        assert!(result.is_ok());
        assert_eq!(4, events.len());
        assert_eq!(Event{id: Id(1), value: Value(42)}, events[0]);
        assert_eq!(Event{id: Id(2), value: Value(43)}, events[1]);
        assert_eq!(Event{id: Id(3), value: Value(44)}, events[2]);
        assert_eq!(Event{id: Id(4), value: Value(45)}, events[3]);
    }

    #[test]
    fn test_publisher() {
        let (polling, input) = MockEventPolling::new();
        let (channel, output) = MockPubChannel::new();
        let mut publisher = EventPublisher::new(polling, channel);
        let mut stop_signal = Signal::new();

        let publisher_stop = stop_signal.clone();
        thread::spawn(move|| {
            publisher.run(publisher_stop);
        });
        
        input.send(Event{id: Id(1), value: Value(0x01020304)}).unwrap();
        input.send(Event{id: Id(2), value: Value(0x02030405)}).unwrap();
        input.send(Event{id: Id(3), value: Value(0x03040506)}).unwrap();

        assert_eq!(vec![0x00, 0x01, 0x01, 0x02, 0x03, 0x04], output.recv().unwrap());
        assert_eq!(vec![0x00, 0x02, 0x02, 0x03, 0x04, 0x05], output.recv().unwrap());
        assert_eq!(vec![0x00, 0x03, 0x03, 0x04, 0x05, 0x06], output.recv().unwrap());

        stop_signal.activate();
    }

    struct MockEventPolling {
        rx: mpsc::Receiver<Event>,
    }

    impl MockEventPolling {
        fn new() -> (MockEventPolling, mpsc::Sender<Event>) {
            let (tx, rx) = mpsc::channel();
            (MockEventPolling {rx}, tx)
        }
    }

    impl EventPolling for MockEventPolling {
        fn poll(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
            loop {
                match self.rx.try_recv() {
                    Ok(event) => events.push(event),
                    Err(_) => break,
                }
            }
            Ok({})
        }
    }

    struct MockPubChannel {
        tx: mpsc::Sender<Vec<u8>>,
    }

    impl MockPubChannel {        
        fn new() -> (MockPubChannel, mpsc::Receiver<Vec<u8>>) {
            let (tx, rx) = mpsc::channel();
            (MockPubChannel{ tx }, rx)
        }
    }

    impl io::Write for MockPubChannel {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { 
            let event = Vec::from(buf);            
            self.tx.send(event).unwrap();
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> { Ok({}) }
    }

    
    impl PubChannel for MockPubChannel {
        fn close(self) -> io::Result<()> { Ok({}) }
    }    
}