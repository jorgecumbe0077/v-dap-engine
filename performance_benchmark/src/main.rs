mod scheduler;

use scheduler::Scheduler;
use std::thread;
use std::time::{Instant, Duration};
use crossbeam_channel::bounded;

#[derive(Clone, Copy)]
pub struct Packet {
    pub id: u64,
    pub priority: u8,
    pub created_at: Option<Instant>,
    pub payload: [u8; 47],
}

fn main() {
    let num_producers: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);

    let iterations = 250_000;
    let total_packets = iterations * num_producers;
    
    let (tx_high, rx_high) = bounded::<Packet>(10000);
    let (tx_low, rx_low) = bounded::<Packet>(10000);

    let start_time = Instant::now();

    let mut handles = vec![];
    for t in 0..num_producers {
        let (th, tl) = (tx_high.clone(), tx_low.clone());
        handles.push(thread::spawn(move || {
            for i in 0..iterations {
                let timestamp = if i % 100 == 0 { Some(Instant::now()) } else { None };
                
                let pkt = Packet { 
                    id: (t * iterations + i) as u64, 
                    priority: (i % 3) as u8, 
                    created_at: timestamp, 
                    payload: [0; 47] 
                };
                
                // Módulo Scheduler abstrai a lógica de roteamento
                Scheduler::dispatch(pkt.priority, &th, &tl, pkt);
            }
        }));
    }
    drop(tx_high); drop(tx_low);

    let consumer = thread::spawn(move || {
        let mut latencies_high = Vec::with_capacity(total_packets / 300);
        let mut latencies_low = Vec::with_capacity(total_packets * 2 / 300);

        loop {
            let mut processed = false;
            for _ in 0..8 { 
                if let Ok(pkt) = rx_high.try_recv() { 
                    if let Some(created) = pkt.created_at {
                        latencies_high.push(created.elapsed()); 
                    }
                    processed = true; 
                } else { break; } 
            }
            for _ in 0..2 { 
                if let Ok(pkt) = rx_low.try_recv() { 
                    if let Some(created) = pkt.created_at {
                        latencies_low.push(created.elapsed()); 
                    }
                    processed = true; 
                } else { break; } 
            }
            if !processed { break; }
        }
        (latencies_high, latencies_low)
    });

    for handle in handles { handle.join().unwrap(); }
    let (mut l_high, mut l_low) = consumer.join().unwrap();
    let total_elapsed = start_time.elapsed();

    let mpps = total_packets as f64 / total_elapsed.as_secs_f64() / 1_000_000.0;
    println!("--- Performance Report (Producers: {}) ---", num_producers);
    println!("Throughput:    {:.2} Mpps", mpps);

    let print_stats = |data: &mut Vec<Duration>, label: &str| {
        data.sort_unstable();
        let len = data.len();
        if len == 0 { return; }
        println!("{}: P50: {:?}, P95: {:?}, P99: {:?}, P99.9: {:?}", 
                 label, data[len * 50 / 100], data[len * 95 / 100], data[len * 99 / 100], data[len * 999 / 1000]);
    };

    print_stats(&mut l_high, "High Priority");
    print_stats(&mut l_low, "Low Priority ");
}