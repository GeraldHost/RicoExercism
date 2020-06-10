/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let s2_chars: Vec<char> = s2.chars().collect();
    match s1.len() == s2.len() {
        true => {
            let hamming =
                s1.chars().enumerate().fold(
                    0,
                    |acc, (i, c)| {
                        if c != s2_chars[i] {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                );
            Some(hamming)
        }
        false => None,
    }
}
