pub fn reply(message: &str) -> &str {
    let is_question = |s: &str| s.trim().chars().last() == Some('?');
    let is_yell = |s: &str| {
        s.len() > 0
            && s.chars().any(|c| c.is_alphabetic())
            && s.chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    };

    if is_question(message) && is_yell(message) {
        return "Calm down, I know what I'm doing!";
    }
    if is_question(message) {
        return "Sure.";
    }
    if is_yell(message) {
        return "Whoa, chill out!";
    }
    if message.trim() == "" {
        return "Fine. Be that way!";
    }
    return "Whatever.";
}
