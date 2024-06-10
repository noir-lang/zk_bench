use dep::std::hash;

global iterations = 100;

fn main(input: [[Field; 2]; iterations]) -> pub [Field; iterations] {
    let mut results: [Field; iterations] = [0; iterations];
    for i in 0..iterations {
        results[i] = hash::pedersen_hash(input[i]);
    }

    results
}
