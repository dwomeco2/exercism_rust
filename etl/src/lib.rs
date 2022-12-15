use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut bt = BTreeMap::new();
    for k in h.keys() {
        let v = h.get(k).unwrap();
        bt.extend(v.iter().map(|&x| (x.to_ascii_lowercase(), *k)));
    }
    bt
}
