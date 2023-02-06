use std::sync::Mutex;
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
fn main() {
    // basic_mutex_check();
    let counter = Rc::new(Mutex::new(0));
    let mut thread_handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let thread_handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        thread_handles.push(thread_handle);
    }
    for handle in thread_handles {
        handle.join();
    }
}
