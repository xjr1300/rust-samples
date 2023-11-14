use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let cloned_a = Arc::clone(&a);
        let handler = thread::spawn(move || {
            for _ in 0..100 {
                let mut a = cloned_a.lock().unwrap();
                *a += 1;
            }
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    assert_eq!(1000, *a.lock().unwrap());
    println!("a = {}", *a.lock().unwrap());
}
