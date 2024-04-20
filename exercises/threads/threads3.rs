// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: &Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) -> (){ 
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    thread::spawn(move || {
        let queue = qc1.lock().unwrap();
        for val in &queue.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
  

    thread::spawn(move || {
        let queue = qc2.lock().unwrap();
        for val in &queue.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
  
}
    
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let queue_length = queue.lock().unwrap().first_half.len() + queue.lock().unwrap().second_half.len();

    send_tx(&queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
