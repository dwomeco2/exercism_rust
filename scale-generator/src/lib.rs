// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale {
    tonic: String,
    interval: String,
}

impl Scale {
    const SHARP: &'static [&'static str; 12] = &[
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    const FLAT: &'static [&'static str; 12] = &[
        "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
    ];

    const USE_SHARP: &'static [&'static str; 12] = &[
        "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
    ];

    const USE_FLAT: &'static [&'static str; 12] = &[
        "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
    ];

    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: tonic.into(),
            interval: intervals.into(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: tonic.into(),
            interval: "".into(),
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut s = Scale::SHARP;
        if Scale::USE_FLAT.contains(&self.tonic.as_str()) {
            s = Scale::FLAT;
        }

        let pos = s
            .iter()
            .position(|&x| x.to_ascii_lowercase() == self.tonic.as_str().to_lowercase())
            .unwrap();

        if self.interval.is_empty() {
            s.iter()
                .cycle()
                .skip(pos)
                .take(13)
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
        } else {
            let indices = self.interval.chars().fold(vec![pos], |mut acc, x| {
                let last = acc.last().unwrap();
                match x {
                    'M' => acc.push(last + 2),
                    'm' => acc.push(last + 1),
                    'A' => acc.push(last + 3),
                    _ => unreachable!(),
                }
                acc
            });
            indices
                .iter()
                .map(|i| i % s.len())
                .map(|i| s.get(i).unwrap().to_string())
                .collect::<Vec<_>>()
        }
    }
}
