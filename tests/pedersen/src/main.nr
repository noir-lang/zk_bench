use dep::std::hash;


fn main(input: [Field; 2]) -> pub Field {
    hash::pedersen_hash(input)
}