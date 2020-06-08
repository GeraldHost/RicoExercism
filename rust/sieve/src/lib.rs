pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result = (2 .. upper_bound + 1).collect::<Vec<u64>>();
    let mut pos = 0;
    while pos < result.len() {
        let prime = result[pos];
        result.retain(|&i| i == prime || i % prime != 0);
        pos += 1;
    }
    result
}
