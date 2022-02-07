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
        n: i128,
    }
    let limit = 2_usize.pow(31_u32) as i128;
    let ans = if -1 * limit <= n && n < limit { "Yes" } else { "No" };
    println!("{}", ans);
}
