fn multi_and<N>(a: [u1; N]) -> u1 {
    let mut result = a[0];
    for i in 1..N {
        result = result & a[i];
    }
    result
}


fn main(a:[u1;3]) -> pub u1 {
    multi_and(a)
}