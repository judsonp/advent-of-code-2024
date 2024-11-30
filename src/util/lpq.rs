use num::PrimInt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LimitedPriorityQueue<P, T>
where
    P: PrimInt + Into<usize> + From<usize>,
{
    size: usize,
    base_priority: P,
    max_priority_skew: P,
    queues: Vec<Vec<T>>,
}

impl<P, T> LimitedPriorityQueue<P, T>
where
    P: PrimInt + Into<usize> + From<usize>,
{
    pub fn new(max_priority_skew: P) -> Self {
        Self {
            size: 0,
            base_priority: P::zero(),
            max_priority_skew,
            queues: num::range(P::zero(), max_priority_skew)
                .map(|_| Vec::new())
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, object: T, priority: P) {
        let mut adjusted_priority = priority - self.base_priority;

        if adjusted_priority >= self.max_priority_skew {
            let shift = adjusted_priority - self.max_priority_skew + P::one();
            for i in num::range(P::zero(), shift) {
                assert!(self.queues[i.into()].is_empty());
            }
            self.queues.rotate_left(shift.into());
            adjusted_priority = adjusted_priority - shift;
            self.base_priority = self.base_priority + shift;
        }

        self.queues[adjusted_priority.into()].push(object);
        self.size += 1;
    }

    pub fn pop(&mut self) -> (T, P) {
        assert!(self.size > 0);
        for i in 0..self.queues.len() {
            if !self.queues[i].is_empty() {
                let object = self.queues[i].pop().unwrap();
                let priority = self.base_priority + i.into();
                self.size -= 1;
                return (object, priority);
            }
        }
        unreachable!();
    }
}
