// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
#[derive(Debug, Clone)]
pub struct Matcher<T>
where
    T: Clone,
{
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T>
where
    T: Clone,
{
    pub fn new(matcher: fn(T) -> bool, subs: &str) -> Matcher<T> {
        Matcher {
            matcher,
            subs: String::from(subs),
        }
    }

    fn substitute(&self, val: T) -> Option<String> {
        if (self.matcher)(val) {
            Some(self.subs.clone())
        } else {
            None
        }
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
pub struct Fizzy<T>
where
    T: Clone,
{
    matchers: Vec<Matcher<T>>,
}

impl<T: std::clone::Clone + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().map(move |val| {
            let subs = self
                .matchers
                .iter()
                .flat_map(|matcher| matcher.substitute(val.clone()))
                .collect::<Vec<String>>();
            if subs.is_empty() {
                val.to_string()
            } else {
                subs.join("")
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Clone>() -> Fizzy<T>
where
    T: Clone + PartialEq + std::ops::Rem<T, Output = T> + ToString,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
