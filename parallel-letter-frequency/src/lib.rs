use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hm = HashMap::<char, usize>::new();

    let mut handles = vec![];
    let input = input.join("");
    let mut chars_list = input.chars();
    let take_size = (input.len() / worker_count) + 1;
    for _ in 0..worker_count {
        let s = chars_list.by_ref().take(take_size).collect::<String>();

        let handle = thread::spawn(move || {
            let mut hm = HashMap::<char, usize>::new();
            s.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .for_each(|x| {
                    *hm.entry(x).or_insert(0) += 1;
                });
            hm
        });
        handles.push(handle);
    }

    for h in handles {
        let thread_result = h.join().unwrap();
        for (key, value) in thread_result.iter() {
            *hm.entry(*key).or_default() += value;
        }
    }

    hm
}
