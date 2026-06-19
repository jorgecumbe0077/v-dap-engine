use v_dap_engine::Transaction;
use pqcrypto_dilithium::dilithium2::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let num_txs = 1000;
    let (pk, sk) = keypair();
    let msg = b"v-dap-engine-tx".to_vec();
    let sig = detached_sign(&msg, &sk);

    println!("{:<10} | {:<15} | {:<10}", "Threads", "Tempo (ms)", "Speedup");
    println!("------------------------------------------");

    // Vamos testar com 1, 2, 4, 8 threads (ajusta conforme o teu CPU)
    let thread_configs = vec![1, 2, 4, 8];
    let mut baseline_time = 0.0;

    for num_threads in thread_configs {
        // Configura o pool de threads do Rayon para este teste
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();

        let start = Instant::now();
        pool.install(|| {
            (0..num_txs).into_par_iter().for_each(|i| {
                let mut tx = Transaction::new(i as u64, pk.clone(), sig.clone(), msg.clone());
                tx.validate();
            });
        });
        
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        
        if num_threads == 1 { baseline_time = duration; }
        let speedup = baseline_time / duration;

        println!("{:<10} | {:<15.2} | {:<10.2}x", num_threads, duration, speedup);
    }
}
