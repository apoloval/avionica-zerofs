//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

/// An object that is doing some work in the background.
///
/// The Worker trait can be extended to support the background execution of
/// tasks. Just implement the `work_loop()` function ensuring it won't get
/// blocked. The worker will loop internally checking if an interruption is
/// requested, and periodically calling to `work_loop()`. So make sure it will
/// return as often as possible.
///
/// The types implementing this trait usually follow this pattern: the `new()`
/// static method is used to create the worker and prepare its internal state.
/// After that, the `spawn()` method can be invoked, consuming the worker
/// instance and returning a `WorkerHandle`. The handle can be used to request
/// the worker to stop doing its work. As simple as that.
///
pub trait Worker : Send + Sized + 'static {

    fn work_loop(&mut self);

    fn spawn(self) -> ShutdownHandle {
        let stop_for_worker = Arc::new(AtomicBool::new(false));
        let stop_for_handle = stop_for_worker.clone();
        let thread_handle = thread::spawn(move || {
            let mut worker = self;
            while !stop_for_worker.load(Ordering::Relaxed) {
                worker.work_loop();
            }
        });
        ShutdownHandle {
            stop: stop_for_handle,
            thread_handle: thread_handle
        }
    }
}

/// The shutdown handle resulting from calling `Worker::spawn()`. It can be
/// used to shut the worker down.
pub struct ShutdownHandle {
    stop: Arc<AtomicBool>,
    thread_handle: thread::JoinHandle<()>,
}

impl ShutdownHandle {
    pub fn shutdown(self) {
        self.stop.store(true, Ordering::Relaxed);
        if self.thread_handle.join().is_err() {
            error!("unexpected error while joining worker thread");
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_woker_operation() {
        let counter = Arc::new(AtomicUsize::new(0));
        let worker = CounterWorker::new(counter.clone());
        let handle = worker.spawn();

        thread::sleep(Duration::from_millis(10));
        handle.shutdown();
        assert!(counter.load(Ordering::Relaxed) > 0);
    }

    struct CounterWorker {
        counter: Arc<AtomicUsize>
    }

    impl Worker for CounterWorker {
        fn work_loop(&mut self) {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    impl CounterWorker {
        fn new(counter: Arc<AtomicUsize>) -> CounterWorker {
            CounterWorker { counter: counter }
        }
    }
}
