use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let chars: Vec<char> = rna.chars().collect();
        let chunks = chars.chunks(3);
        let codons = chunks
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>();
        let mut result : Vec<&'a str> = Vec::new();
        for codon in codons {
            let maybe_protein = self.map.get(codon.as_str());
            match maybe_protein {
                | Some(&"stop codon") => { break; }
                | Some(protein) => { result.push(protein); }
                | None => { return None; }
            }
        }
        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map: HashMap<&'a str, &'a str> = HashMap::new();
    for pair in pairs {
        map.insert(pair.0, pair.1);
    }
    CodonsInfo { map }
}
