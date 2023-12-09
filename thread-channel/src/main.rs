use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("few"),
            String::from("more"),
            String::from("value"),
            String::from("from other thread")
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("got {received} from spawned thread");
    }

}
