use std::{cmp::Reverse, collections::BinaryHeap};

use crate::collector::Collector;

#[derive(Clone)]
pub struct TopKCollector<T: Clone + Ord> {
    k: usize,
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Clone + Ord> Collector<T, Vec<T>> for TopKCollector<T> {
    fn collect(&mut self, item: T) {
        // push the item onto the heap
        self.heap.push(Reverse(item));

        // if the heap is larger than k, pop the smallest item
        if self.heap.len() > self.k {
            self.heap.pop();
        }
    }

    fn result(&mut self) -> Vec<T> {
        self.heap.iter().cloned().map(|item| item.0).collect()
    }
}

impl<T: Clone + Ord> TopKCollector<T> {
    pub fn new(k: usize) -> Self {
        TopKCollector {
            k,
            heap: BinaryHeap::with_capacity(k),
        }
    }
}
