use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct Signal {
    value: Arc<AtomicBool>,
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            value: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn status(&self) -> bool {
        self.value.load(Ordering::Relaxed)
    }

    pub fn activate(&mut self) {
        self.set(true);
    }

    pub fn deactivate(&mut self) {
        self.set(false);
    }

    fn set(&mut self, value: bool) {
        self.value.store(value, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use std::thread::spawn;
    use super::*;

    #[test]
    fn test_signal() {
        let mut a = Signal::new();
        a.deactivate();

        let mut b = a.clone();
        let child = spawn(move|| {
            b.activate();
        });
        let result = child.join();

        assert!(result.is_ok());
        assert!(a.status());
    }
}