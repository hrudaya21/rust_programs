use std::sync::mpsc; // Multi Producer and Single Consumer
use std::{thread, time::Duration};

fn single_message_send() {
    let (tx, rx) = mpsc::channel(); // tx => Sender, rx => receiver
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
        // println!("Msg is {}", msg); // This will give the compilation error,  msg owner is transfer to receiver
    });

    let received = rx.recv().unwrap(); // try_recv also another API where it can handle if sender has no data to send.
    println!("Got: {}", received);
}
fn multi_message_send_single_receive() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
fn mutliple_message_send_from_multiple_thread_received() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
});
    for received in rx {
        println!("Got: {}", received);
    }
}
fn main() {
    // single_message_send();
    // multi_message_send_single_receive();
    mutliple_message_send_from_multiple_thread_received();
}
