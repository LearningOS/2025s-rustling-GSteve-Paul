/*
    heap
    This question requires you to implement a binary heap function
*/

use std::clone;
use std::cmp::Ord;
use std::default::Default;
use std::mem::swap;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.resize(self.count + 1, T::default());
        self.items[self.count] = value;
        let mut idx = self.count;
        while idx != 1 {
            let pa_idx = self.parent_idx(idx);
            let pa = &self.items[pa_idx];
            match (self.comparator)(&self.items[idx], pa) {
                true => {
                    self.items.swap(idx, pa_idx);
                }
                false => break,
            }
            idx = pa_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn present(&self, idx: usize) -> bool {
        idx <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let lidx = self.left_child_idx(idx);
        let ridx = self.right_child_idx(idx);

        if !self.present(lidx) && !self.present(ridx) {
            return usize::MAX;
        }

        if !self.present(lidx) {
            return ridx;
        }

        if !self.present(ridx) {
            return lidx;
        }

        if (self.comparator)(&self.items[lidx], &self.items[ridx]) {
            return lidx;
        } else {
            return ridx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone + std::fmt::Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => {
                let ret = self.items[1].clone();
                self.items.swap(1, self.count);
                self.count -= 1;
                for mut idx in (1..=(self.parent_idx(self.count))).rev() {
                    loop {
                        let mn_child = self.smallest_child_idx(idx);
                        if mn_child == usize::MAX {
                            break;
                        }
                        match (self.comparator)(&self.items[mn_child], &self.items[idx]) {
                            true => {
                                self.items.swap(idx, mn_child);
                                idx = mn_child
                            }
                            false => break,
                        }
                    }
                }
                Some(ret)
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone + std::fmt::Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
