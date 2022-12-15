pub fn encode(source: &str) -> String {
    let mut v: Vec<(char, i32)> = vec![];
    for c in source.chars() {
        if let Some(g) = v.last_mut() {
            if g.0 == c {
                g.1 = g.1 + 1;
                continue;
            }
        }

        v.push((c, 1));
    }
    v.iter()
        .map(|&x| match x.1 {
            1 => x.0.to_string(),
            _ => x.1.to_string() + x.0.to_string().as_str(),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut v: Vec<String> = vec![];
    for c in source.chars() {
        if let Some(g) = v.last_mut() {
            if g.parse::<i32>().is_ok() {
                g.push_str(c.to_string().as_str());
                continue;
            }
        }
        v.push(c.to_string());
    }
    v.iter()
        .map(|s| {
            if s.len() == 1 {
                return s.to_owned();
            } else {
                let (count, c) = s.split_at(s.len() - 1);
                if let Ok(num) = count.parse::<i32>() {
                    return c.repeat(num as usize).to_string();
                }
            }
            String::new()
        })
        .collect::<String>()
}
