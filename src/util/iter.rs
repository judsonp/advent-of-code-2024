pub trait CountIf<T> {
    fn count_if<F>(self, predicate: F) -> usize
    where
        T: Iterator,
        F: FnMut(T::Item) -> bool;
}

impl<T> CountIf<T> for T
where
    T: Iterator,
{
    fn count_if<F>(self, predicate: F) -> usize
    where
        F: FnMut(T::Item) -> bool,
    {
        self.map(predicate).filter(|&b| b).count()
    }
}
