use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    hm: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.hm.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let mut v = vec![];
        let mut i: usize = 0;
        while i < rna.len() {
            if i + 3 > rna.len() {
                return None;
            }
            match self.name_for(&rna[i..i + 3]) {
                None => return None,
                Some("stop codon") => break,
                Some(n) => v.push(n),
            }
            i += 3;
        }
        Some(v)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        hm: pairs.into_iter().collect(),
    }
}
