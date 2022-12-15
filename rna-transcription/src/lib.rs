use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(x) = dna.find(|x| x != 'A' && x != 'C' && x != 'G' && x != 'T') {
            return Err(x);
        }
        Ok(Dna { dna: dna.into() })
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            self.dna
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect::<String>()
                .deref(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(x) = rna.find(|x| x != 'A' && x != 'C' && x != 'G' && x != 'U') {
            return Err(x);
        }
        Ok(Rna { rna: rna.into() })
    }
}
