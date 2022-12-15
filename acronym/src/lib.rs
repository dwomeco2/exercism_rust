pub fn abbreviate(phrase: &str) -> String {
    if phrase == "" {
        return String::new();
    }
    if let Some((s, _)) = phrase.split_once(' ') {
        if s.chars().all(|c| c.is_alphabetic() && c.is_uppercase()) && s.ends_with(":") {
            return s.strip_suffix(":").unwrap().into();
        }
    }
    let new = phrase.replace(|c| c == '_' || c == '-' || c == ',', " ");
    let new = new.replace("'s", "");
    let new = new
        .split_ascii_whitespace()
        .map(|x| {
            let t = x
                .char_indices()
                .filter(|c| c.1.is_alphabetic() && c.1.is_uppercase())
                .collect::<Vec<_>>();
            let is_camelcase = t.len() > 1 && t[0].0.abs_diff(t[1].0) > 1;
            if is_camelcase {
                t.iter().map(|c| c.1).collect::<Vec<_>>()
            } else {
                vec![x.chars().next().unwrap()]
            }
        })
        .flatten()
        .collect::<String>();
    new.to_uppercase()
}
