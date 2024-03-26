use dep::std;

fn main(x: [u8; 2], result: [u8; 32]) {
    // We use the `as` keyword here to denote the fact that we want to take just the first byte from the x Field
    // The padding is taken care of by the program
    let digest = std::hash::sha256(x);
    std::println(digest);
    assert(digest == result);
}
