use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn basic_mutex_check() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
fn share_state() {
    // In Multi threaded reference counting, Arc smart pointer to be used.
    // Rc smart pointer can be used in single threaded scenario
    let counter = Arc::new(Mutex::new(0));
    let mut thread_handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let thread_handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        thread_handles.push(thread_handle);
    }
    
    for handle in thread_handles {
        handle.join().unwrap();
    }
    println!("Value: {}", *counter.lock().unwrap());
}
fn main() {
    // basic_mutex_check();
    share_state();
}
