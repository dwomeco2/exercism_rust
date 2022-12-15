pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    list.windows(2)
        .map(|x| {
            return format!("For want of a {} the {} was lost.", x[0], x[1]);
        })
        .chain([format!("And all for the want of a {}.", list[0])])
        .collect::<Vec<_>>()
        .join("\n")
}
