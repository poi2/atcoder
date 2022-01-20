#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, k: usize,
        abn: [[usize; 2]; n],
    }
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((abn[i][1], i));
    }
    let mut ans = 0;
    for _ in 0..k {
        if let Some((point, idx)) = heap.pop() {
            ans += point;
            if idx != n + 1 {
                let new_point = abn[idx][0] - abn[idx][1];
                heap.push((new_point, n + 1));
            }
        }
    }
    println!("{}", ans);
}
