// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    matcher: Box<dyn Fn(T) -> bool + 'a>,
    subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }

    pub fn call(&self, ele: T) -> bool {
        (self.matcher)(ele)
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<'a, T>(Vec<Matcher<'a, T>>);

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Clone + Copy + 'a,
{
    pub fn new() -> Self {
        Fizzy(vec![])
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.0.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = T> + 'a,
    {
        let matchers = self.0;
        iter.into_iter().map(move |e| {
            let s = matchers
                .iter()
                .map(|m| match m.call(e) {
                    true => m.subs.clone(),
                    false => "".to_string(),
                })
                .collect::<String>();
            if s.is_empty() {
                return e.to_string();
            }
            s
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: ToString + Clone + Copy + Rem<Output = T> + PartialEq + 'a,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
