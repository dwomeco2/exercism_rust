/// What should the type of _function be?
pub fn map<T, F, O>(input: Vec<T>, function: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    input.into_iter().map(function).collect()
}
