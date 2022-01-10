#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [usize; n],
    }

    let (left, _right) = pn.split_at(k);

    let mut heap = BinaryHeap::new();
    for i in left {
        heap.push(Reverse(*i));
    }

    let mut border = match heap.peek() {
        Some(Reverse(j)) => *j,
        _ => 0 as usize,
    };
    println!("{}", border);

    for i in k..n {
        let pni = pn[i];
        if border < pni {
            heap.pop();
            heap.push(Reverse(pni));
            border = match heap.peek() {
                Some(Reverse(j)) => *j,
                _ => 0 as usize,
            };
        }
        println!("{}", border);

    }
}

