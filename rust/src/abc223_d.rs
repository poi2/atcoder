#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// NOTE トポロジカルソートを辞書順に実行する
fn main() {
    input! {
        n: usize, m: usize,
        abm: [(usize, usize); m],
    }

    let mut inflow = vec![0_usize; n + 1];
    let mut dists = vec![vec![]; n + 1];
    for (a, b) in abm {
        inflow[b] += 1;
        dists[a].push(b);
    }

    let mut heap = BinaryHeap::new();
    for i in 1..=n {
        if inflow[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut sorted = vec![];
    while let Some(Reverse(i)) = heap.pop() {
        sorted.push(i);
        for &j in &dists[i] {
            inflow[j] -= 1;
            if inflow[j] == 0 {
                heap.push(Reverse(j));
            }
        }
    }

    // NOTE DAG ではないときはトポロジカルソートが終了せず、sorted.len() != n となる。
    // なので sorted.len() == 0 のとき -1 とするのは誤り。
    if sorted.len() == n {
        println!("{}", sorted.iter().join(" "));
    } else {
        println!("{}", -1);
    }
}
