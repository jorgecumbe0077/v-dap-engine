
use std::time::{Instant, Duration};
use crossbeam_channel::unbounded;
use std::thread;

#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy)]
pub struct Packet {
    pub id: u64,
    pub priority: u8,
    pub created_at: Instant,
    pub payload: [u8; 47],
}

fn main() {
    // 1. Parametrizável via linha de comando
    let num_producers: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);

    let iterations = 250_000;
    let total_packets = iterations * num_producers;
    
    let (tx_high, rx_high) = unbounded::<Packet>();
    let (tx_low, rx_low) = unbounded::<Packet>();

    // Início da medição de tempo total para Throughput
    let start_time = Instant::now();

    let mut handles = vec![];
    for t in 0..num_producers {
        let (th, tl) = (tx_high.clone(), tx_low.clone());
        handles.push(thread::spawn(move || {
            for i in 0..iterations {
                let pkt = Packet { 
                    id: (t * iterations + i) as u64, 
                    priority: (i % 3) as u8, 
                    created_at: Instant::now(), 
                    payload: [0; 47] 
                };
                if pkt.priority == 0 { th.send(pkt).unwrap(); } else { tl.send(pkt).unwrap(); }
            }
        }));
    }
    drop(tx_high); drop(tx_low);

    let consumer = thread::spawn(move || {
        let mut latencies_high = Vec::with_capacity(total_packets / 3);
        let mut latencies_low = Vec::with_capacity(total_packets * 2 / 3);

        loop {
            let mut processed = false;
            // WRR Weights: High 8, Low 2
            for _ in 0..8 { if let Ok(pkt) = rx_high.try_recv() { latencies_high.push(pkt.created_at.elapsed()); processed = true; } else { break; } }
            for _ in 0..2 { if let Ok(pkt) = rx_low.try_recv() { latencies_low.push(pkt.created_at.elapsed()); processed = true; } else { break; } }
            if !processed { break; }
        }
        (latencies_high, latencies_low)
    });

    for handle in handles { handle.join().unwrap(); }
    let (mut l_high, mut l_low) = consumer.join().unwrap();
    let total_elapsed = start_time.elapsed();

    // Cálculo de Throughput
    let mpps = total_packets as f64 / total_elapsed.as_secs_f64() / 1_000_000.0;
    println!("--- Performance Report (Producers: {}) ---", num_producers);
    println!("Throughput:    {:.2} Mpps", mpps);

    // Cálculo de Percentis (fora do tempo de processamento)
    let print_stats = |data: &mut Vec<Duration>, label: &str| {
        data.sort_unstable();
        let len = data.len();
        if len == 0 { return; }
        println!("{}: P50: {:?}, P95: {:?}, P99: {:?}, P99.9: {:?}", 
                 label, 
                 data[len * 50 / 100], 
                 data[len * 95 / 100], 
                 data[len * 99 / 100], 
                 data[len * 999 / 1000]);
    };

    print_stats(&mut l_high, "High Priority");
    print_stats(&mut l_low, "Low Priority ");
}
