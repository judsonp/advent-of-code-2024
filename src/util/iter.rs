use rayon::iter::ParallelIterator;

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

pub trait CountIfParallel<T> {
    fn count_if<F>(self, predicate: F) -> usize
    where
        T: ParallelIterator,
        F: Fn(T::Item) -> bool + Sync + Send;
}

impl<T> CountIfParallel<T> for T
where
    T: ParallelIterator,
{
    fn count_if<F>(self, predicate: F) -> usize
    where
        F: Fn(T::Item) -> bool + Sync + Send,
    {
        self.map(predicate).filter(|&b| b).count()
    }
}
