use dep::std::hash::mimc;


fn main(input: [Field; 2]) -> pub Field {
    mimc::mimc_bn254(input)
}