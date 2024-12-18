use std::cmp::Ordering;

pub mod bbox;
pub mod direction;
pub mod geom;
pub mod grid;
pub mod iter;
pub mod lpq;
pub mod point;

pub struct DistanceState<D, T>
where
    D: Eq + Ord,
{
    pub distance: D,
    pub state: T,
}

impl<D, T> PartialEq for DistanceState<D, T>
where
    D: Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<D, T> Eq for DistanceState<D, T> where D: Eq + Ord {}

impl<D, T> Ord for DistanceState<D, T>
where
    D: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl<D, T> PartialOrd for DistanceState<D, T>
where
    D: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<D, T> DistanceState<D, T>
where
    D: Eq + Ord,
{
    pub fn new(distance: D, state: T) -> Self {
        Self { distance, state }
    }
}
