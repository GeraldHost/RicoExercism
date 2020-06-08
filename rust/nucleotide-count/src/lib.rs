use std::collections::HashMap;

const VALID_CHARS: [char; 4] = ['A', 'G', 'C', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_CHARS.contains(&nucleotide) {
        return Err('X')
    }

    dna.chars().fold(Ok(0), |acc, x| {
        let count = acc?;
        if !VALID_CHARS.contains(&x) { return Err(x); }
        if x == nucleotide { Ok(count + 1) } else { acc }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm = HashMap::new();
    hm.insert('A', 0);
    hm.insert('G', 0);
    hm.insert('C', 0);
    hm.insert('T', 0);

    for c in dna.chars() {
        if let Some(size) = hm.get_mut(&c) {
            *size += 1;
        } else {
            return Err(c);
        }
    }

    Ok(hm)
}
