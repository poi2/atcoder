#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        h: usize, w: usize,
        aij: [[usize; w]; h],
    }

    let ans = (0..w).map(|j|
        (0..h).map(|i| aij[i][j].to_string()).join(" ")
    ).join("\n");
    println!("{}", ans);
}
