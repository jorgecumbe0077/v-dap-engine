use std::time::Instant;
use crossbeam_channel::unbounded;
use dashmap::DashMap;
use std::sync::Arc;
use std::thread;

fn main() {
    let iterations = 2_500_000;
    let num_producers = 4;

    let map = Arc::new(DashMap::new());
    let start_map = Instant::now();
    let mut handles_map = vec![];
    for t in 0..num_producers {
        let m = Arc::clone(&map);
        handles_map.push(thread::spawn(move || {
            for i in 0..iterations { m.insert(t * iterations + i, i); }
        }));
    }
    for handle in handles_map { handle.join().unwrap(); }
    println!("DashMap (4 Prods): {:?} para {} ops", start_map.elapsed(), iterations * num_producers);

    let (tx, rx) = unbounded();
    let start_queue = Instant::now();
    let mut handles_queue = vec![];
    for _ in 0..num_producers {
        let tx_clone = tx.clone();
        handles_queue.push(thread::spawn(move || {
            for i in 0..iterations { let _ = tx_clone.send(i); }
        }));
    }
    drop(tx);
    let consumer = thread::spawn(move || {
        let mut count = 0;
        while let Ok(_) = rx.recv() { count += 1; }
        count
    });
    for handle in handles_queue { handle.join().unwrap(); }
    let total_received = consumer.join().unwrap();
    println!("MPSC Lock-Free (4 Prods): {:?} para {} ops", start_queue.elapsed(), total_received);
}
