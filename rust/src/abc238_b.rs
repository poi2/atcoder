#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut solver = Solver::new();
    for a in an { solver.add(a); }
    println!("{}", solver.ans());
}

struct Solver {
    sum: usize,
    heap: BinaryHeap<usize>,
    rad: usize,
}
impl Solver {
    pub fn new() -> Self {
        let rad = 360;
        let mut heap = BinaryHeap::new();
        heap.push(0);
        heap.push(rad);
        return Self { sum: 0, heap: heap, rad: rad };
    }

    fn add(&mut self, a: usize) -> () {
        self.sum += a;
        self.sum %= self.rad;
        self.heap.push(self.sum);
    }

    fn ans(&mut self) -> usize {
        let mut vec = self.heap
            .iter()
            .map(|v| *v)
            .collect::<Vec<usize>>();
        vec.sort();

        let ans = vec
            .iter()
            .tuple_windows()
            .map(|(first, second)| second - first)
            .max()
            .unwrap();

        return ans;
    }
}
