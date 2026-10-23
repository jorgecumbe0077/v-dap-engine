mod tsc;
use clap::Parser;
use v_dap_engine::tail_detector::{RobustReference, TailClassification};
use pqcrypto_dilithium::dilithium2 as dilithium;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 100_000)]
    operations: usize,
}

fn main() {
    let args = Args::parse();

    if args.operations == 0 {
        eprintln!("Erro: --operations deve ser maior que zero.");
        std::process::exit(2);
    }

    println!(
        "Iniciando execução | Modo: BASELINE | Operações: {}",
        args.operations
    );

    let start = Instant::now();
    let tsc_start = tsc::read();
    let mut processed = 0;

    // Instrumentação da distribuição por operação:
    // pré-alocada para não exigir crescimento do buffer durante o loop.
    let mut op_latencies: Vec<u64> = Vec::with_capacity(args.operations);
    let mut op_aux_migrations: usize = 0;

    // Referência robusta congelada a partir do baseline C0.
    let tail_reference = RobustReference {
        median: 1_682_149,
        mad: 531_957,
        k: 5,
    };
    let tail_threshold = tail_reference.threshold();
    let mut tail_count: usize = 0;

    for _ in 0..args.operations {
        let op_tsc_start = tsc::read();

        // Caminho criptográfico preservado exatamente:
        // keypair() + detached_sign(b"data", &sk)
        let (_, sk) = dilithium::keypair();
        let _ = dilithium::detached_sign(b"data", &sk);

        let op_tsc_end = tsc::read();

        let op_delta = op_tsc_end.ticks.saturating_sub(op_tsc_start.ticks);

        op_latencies.push(op_delta);

        if tail_reference.classify(op_delta) == TailClassification::Tail {
            tail_count += 1;
        }

        if op_tsc_start.aux != op_tsc_end.aux {
            op_aux_migrations += 1;
        }

        processed += 1;
    }

    if !op_latencies.is_empty() {
        let mut sorted = op_latencies.clone();
        sorted.sort_unstable();

        let percentile = |p: f64| -> u64 {
            let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
            sorted[idx]
        };

        println!("--- DISTRIBUIÇÃO TSC POR OPERAÇÃO ---");
        println!("Samples: {}", sorted.len());
        println!("P50:    {} ticks", percentile(0.50));
        println!("P95:    {} ticks", percentile(0.95));
        println!("P99:    {} ticks", percentile(0.99));
        println!("P99.9:  {} ticks", percentile(0.999));
        println!("MAX:    {} ticks", sorted[sorted.len() - 1]);
        println!("AUX migrations: {}", op_aux_migrations);
        println!("Tail threshold: {} ticks", tail_threshold);
        println!("Tail events: {} / {}", tail_count, op_latencies.len());
    }

    // Persistência offline dos deltas brutos.
    // A escrita ocorre somente após o loop de medição.
    let raw_path = "v03_latencies_raw.txt";
    let raw_data = op_latencies
        .iter()
        .map(|ticks| ticks.to_string())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    std::fs::write(raw_path, raw_data)
        .expect("falha ao escrever latências brutas");

    println!("Raw latencies: {} | Samples: {}", raw_path, op_latencies.len());

    let tsc_end = tsc::read();
    let total_time = start.elapsed().as_secs_f64();
    let tps = processed as f64 / total_time;

    println!("TSC start: {} | AUX: {}", tsc_start.ticks, tsc_start.aux);
    println!("TSC end:   {} | AUX: {}", tsc_end.ticks, tsc_end.aux);
    println!("TSC delta: {} ticks | same AUX: {}", 
             tsc_end.ticks.saturating_sub(tsc_start.ticks),
             tsc_start.aux == tsc_end.aux);

    println!("--- RESULTADOS DE ENGENHARIA ---");
    println!("Throughput: {:.2} ops/s", tps);
    println!("Final Phi (Estabilidade): {:.4}", (total_time / processed as f64) / 0.000013);
}
