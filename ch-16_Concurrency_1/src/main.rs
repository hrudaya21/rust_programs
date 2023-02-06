use std::{thread, time::Duration};

fn thread_without_param_passing() {
    let thread_handle = thread::spawn(|| {
        for i in 1 .. 10 {
            println!("Thread: Hi Number {}, is spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1 .. 5 {
        println!("Main Thread: Hi Number {}, is in main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread_handle.join().unwrap();
}
fn thread_with_param_passing() {
    let v = vec![1, 2, 3];
    let thread_handle = thread::spawn(move || {
        println!("Value of the vector {:?}", v);
    });
    thread_handle.join().unwrap();
}

fn main() {
    // thread_without_param_passing();
    thread_with_param_passing();
}
