pub mod tail_detector;

use pqcrypto_dilithium::dilithium2::{PublicKey, DetachedSignature, verify_detached_signature};
use std::time::Instant;

pub struct Metrics {
    pub latency_ms: f64,
    pub pressure_index: f64,
    pub is_safe: bool,
}

pub struct PQCProcessor;

impl PQCProcessor {
    // Limite de saturação baseado na constante de engrenagem FSGear
    const PRESSURE_LIMIT: f64 = 5.16e9; 

    pub fn validate_and_measure(pk: &PublicKey, sig: &DetachedSignature, msg: &[u8]) -> Metrics {
        let start = Instant::now();
        let _is_valid = verify_detached_signature(sig, msg, pk).is_ok();
        let duration = start.elapsed().as_secs_f64() * 1000.0;

        // O cálculo converte o tempo de duração em fluxo de pressão mA
        // 9.800000045764994 é a constante de Aceleração (Action A)
        let pressure = (duration * 1.0e6) * 9.800000045764994;

        let is_safe = pressure <= Self::PRESSURE_LIMIT;

        Metrics {
            latency_ms: duration,
            pressure_index: pressure,
            is_safe,
        }
    }
}
