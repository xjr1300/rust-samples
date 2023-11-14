use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(AtomicI32::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let cloned_a = a.clone();
        let handler = thread::spawn(move || {
            for _ in 0..100 {
                cloned_a.fetch_add(1, Ordering::Relaxed);
            }
        });
        handlers.push(handler)
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    assert_eq!(1000, a.load(Ordering::Relaxed));
}
