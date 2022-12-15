// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows = input.lines().collect::<Vec<_>>();
    if rows.len() % 4 != 0 || rows.len() == 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    }
    if rows[0].len() % 3 != 0 || rows[0].len() == 0 {
        return Err(Error::InvalidColumnCount(rows[0].len()));
    }

    let result = rows
        .chunks(4)
        .map(|four| four.split_last().unwrap().1)
        .map(|three| {
            let x = three
                .iter()
                .map(|&line| line.as_bytes().chunks(3).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            (0..rows[0].len() / 3)
                .map(|i| ocr(&[x[0][i], x[1][i], x[2][i]].concat()))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(",");
    Ok(result)
}

fn ocr<'a, 'b>(target: &'a [u8]) -> &'b str {
    match target {
        [32, 32, 32, 32, 32, 124, 32, 32, 124] => "1",
        [32, 95, 32, 32, 95, 124, 124, 95, 32] => "2",
        [32, 95, 32, 32, 95, 124, 32, 95, 124] => "3",
        [32, 32, 32, 124, 95, 124, 32, 32, 124] => "4",
        [32, 95, 32, 124, 95, 32, 32, 95, 124] => "5",
        [32, 95, 32, 124, 95, 32, 124, 95, 124] => "6",
        [32, 95, 32, 32, 32, 124, 32, 32, 124] => "7",
        [32, 95, 32, 124, 95, 124, 124, 95, 124] => "8",
        [32, 95, 32, 124, 95, 124, 32, 95, 124] => "9",
        [32, 95, 32, 124, 32, 124, 124, 95, 124] => "0",
        _ => "?",
    }
}
