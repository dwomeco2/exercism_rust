use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if let Some(c) = (nucleotide.to_string() + dna)
        .chars()
        .find(|&c| c != 'A' && c != 'C' && c != 'G' && c != 'T')
    {
        return Err(c);
    }
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    ['A', 'T', 'C', 'G']
        .iter()
        .map(|&c| count(c, dna).and_then(|n| Ok((c, n))))
        .collect()
}
