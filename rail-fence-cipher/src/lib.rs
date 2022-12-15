pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut vec = vec![vec![]; self.0 as usize];
        let mut seq = self.rail_seq();

        for c in text.chars() {
            vec[seq.next().unwrap()].push(c);
        }
        vec.iter().map(String::from_iter).collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut vec = vec![vec![]; self.0 as usize];
        let seq = self.rail_seq().take(cipher.len()).collect::<Vec<_>>();
        let mut cip = cipher;
        for i in 0..self.0 {
            let count = seq.iter().filter(|&&x| x == i as usize).count();
            let (s1, s2) = cip.split_at(count);
            cip = s2;
            vec[i as usize] = Vec::from_iter(s1.chars());
        }

        seq.iter().map(|&i| vec[i].remove(0)).collect()
    }

    fn rail_seq(&self) -> impl Iterator<Item = usize> + '_ {
        let cycle_len = self.0 + (self.0 - 2);
        let mid = (cycle_len / 2) as i32;
        let range = if self.0 != 1 { -mid..mid } else { 0..1 };
        range
            .cycle()
            .map(move |i| (i.abs() - mid).unsigned_abs() as usize)
    }
}
