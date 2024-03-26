mod circom;
mod noir;
mod suite;
mod utils;
fn main() {
    println!("Benchmarking...");
    suite::run().unwrap();
}
