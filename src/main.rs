mod circom;
mod noir;
mod suite;
fn main() {
    println!("Benchmarking...");
    suite::run().unwrap();
}
