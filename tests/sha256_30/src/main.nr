use dep::std::hash::sha256;

fn main(input: [[u8; 2]; 30]) -> pub [[u8; 32]; 30] {
    let mut results: [[u8; 32]; 30] = [[0; 32]; 30];
    for i in 0..30 {
        results[i] = sha256(input[i]);
    }

    results
}
