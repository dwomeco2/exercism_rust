#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len_f = first_list.len();
    let len_s = second_list.len();

    if len_f == 0 && len_s == 0 {
        return Comparison::Equal;
    }
    if len_f == 0 {
        return Comparison::Sublist;
    }
    if len_s == 0 {
        return Comparison::Superlist;
    }
    if len_f == len_s {
        if first_list.starts_with(second_list) && first_list.ends_with(second_list) {
            return Comparison::Equal;
        }
    }

    if len_f <= len_s {
        return comp(first_list, second_list);
    } else {
        let r = comp(second_list, first_list);
        if r == Comparison::Sublist {
            return Comparison::Superlist;
        }
        return r;
    }
}

fn comp<T: PartialEq + std::fmt::Debug>(l1: &[T], l2: &[T]) -> Comparison {
    if l2.windows(l1.len()).any(|x| x == l1) {
        return Comparison::Sublist;
    } else {
        return Comparison::Unequal;
    }
}
