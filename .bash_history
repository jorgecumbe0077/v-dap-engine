cd "/mnt/c/Users/Jeanie/OneDrive/Ambiente de Trabalho/ZeeperPay"
sudo apt update
sudo apt install build-essential linux-tools-common linux-tools-generic linux-tools-$(uname -r) curl -y
sudo apt install build-essential linux-tools-virtual linux-tools-generic curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
cargo check
cargo build --release
./target/release/zeeperpay_pqc
ls target/release/
cargo build --release --target x86_64-unknown-linux-gnu
perf stat ./target/x86_64-unknown-linux-gnu/release/zeeperpay_pqc
ls target/x86_64-unknown-linux-gnu/release/
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
sudo perf c2c record -- ./target/x86_64-unknown-linux-gnu/release/stress_test
sudo perf stat -e cache-references,cache-misses,LLC-loads,LLC-load-misses,LLC-stores,LLC-store-misses ./target/x86_64-unknown-linux-gnu/release/stress_test
find src/ -name "*.rs"
cat src/lib.rs
cargo build --release --target x86_64-unknown-linux-gnu
cargo add dashmap parking_lot
cargo build --release --target x86_64-unknown-linux-gnu
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
cargo build --release --target x86_64-unknown-linux-gnu
cargo clean
cargo build --release --target x86_64-unknown-linux-gnu
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
# 1. Mostra a versão do teu código (para provar que é Lock-Free)
cat src/lib.rs | head -n 25
# 2. Executa o teste de stress com perf para capturar os dados atuais
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
cd /mnt/c/Users/Jeanie/OneDrive/Ambiente\ de\ Trabalho/ZeeperPay
# Limpa os artefatos de build para garantir uma compilação limpa
cargo clean
# Compila a versão otimizada (release)
cargo build --release --target x86_64-unknown-linux-gnu
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
cargo build --release --target x86_64-unknown-linux-gnu
sudo perf stat ./target/x86_64-unknown-linux-gnu/release/stress_test
cargo build --release --target x86_64-unknown-linux-gnu
cargo clean
cargo build --release --bin stress_test
sudo perf stat ./target/release/stress_test
lscpu -e
cargo build --release --bin stress_test
./target/release/stress_test
cargo build --release --bin stress_test
./target/release/stress_test
cargo build --release --bin stress_test
./target/release/stress_test
cargo run --release
cd performance_benchmark
cargo new performance_benchmark
cd performance_benchmark
nano Cargo.toml
nano src/main.rs
cargo run --release
nano src/main.rs
cargo run --release
cargo add crossbeam-channel
cargo add dashmap
cargo run --release
src/main.rs
cargo run --release
nano src/main.rs
perf stat -d cargo run --release
echo '' > src/main.rs
cat <<EOF > src/main.rs
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
EOF

perf stat -d cargo run --release
cargo build --release
perf stat -d ./target/release/performance_benchmark
sudo perf c2c record ./target/release/performance_benchmark
perf stat -e LLC-load-misses,LLC-loads,LLC-store-misses,LLC-stores ./target/release/performance_benchmark
perf record -g ./target/release/performance_benchmark
perf report
cd ~/performance_benchmark
code .
cargo clean
cargo build --release
sudo perf record -g ./target/release/performance_benchmark
cat src/main.rs
use std::thread;
use std::time::{Instant, Duration};
use crossbeam::channel::{bounded, Sender, Receiver};
// Otimização: Adicionamos Copy e tamanho fixo para performance máxima
#[derive(Clone, Copy)]
pub struct Packet {
}
fn main() {
}
cat src/main.rs
cargo build --release
cargo clean
cargo update
cargo build --release
./target/release/performance_benchmark
cargo build --release
./target/release/performance_benchmark
cargo build --release
./target/release/performance_benchmark [numero_de_produtores]
cargo build --release
git add .
git rm --cached performance_benchmark
rm -rf performance_benchmark/.git
git add .
git status
git commit -m "v1.0.0: release production-oriented mempool infrastructure"
git config --global user.email "jorge.cumbe@example.com"
git config --global user.name "Jorge Cumbe"
git commit -m "v1.0.0: release production-oriented mempool infrastructure"
git tag v1.0.0
git push
git remote add origin URL_DO_TEU_REPOSITORIO
git push -u origin main
git push origin v1.0.0
git remote remove origin
git remote add origin https://github.com/jorgecumbe0077/v-dap-engine.git
git push origin v1.0.0
git rm -r --cached .rustup/
git rm -r --cached .vscode-server/
# Se houver outros ficheiros grandes, remove-os da mesma forma
git add .gitignore
git commit -m "fix: ignore toolchain and server files to fix large file upload error"
git push origin main
git push origin v1.0.0
# Remove as pastas do índice (staging)
git rm -r --cached .rustup/
git rm -r --cached .vscode-server/
# Confirma a remoção com um novo commit
git commit -m "chore: remove large toolchain and server files from repository"
# Remove todo o histórico de commits (mantendo os ficheiros na pasta)
rm -rf .git
git init
git add .
git commit -m "v1.0.0: Initial release of v-dap-engine"
# Reconecta ao GitHub (substitui pela tua URL real)
git remote add origin https://github.com/jorgecumbe0077/v-dap-engine.git
# Força o push para sobrescrever o repositório no GitHub (cuidado: isto apaga o histórico anterior no GitHub)
git push -u origin --force --all
# Remove todo o histórico de commits (mantendo os ficheiros na pasta)
rm -rf .git
git init
git add .
