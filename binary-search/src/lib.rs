use std::cmp::Ordering;

pub fn find<T, R>(array: T, key: R) -> Option<usize>
where
    R: Ord,
    T: AsRef<[R]>,
{
    let a = array.as_ref();
    let mid = a.len() / 2;
    match key.cmp(a.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&a[0..mid], key),
        Ordering::Greater => find(&a[mid + 1..], key).and_then(|x| Some(x + mid + 1)),
    }
}
