#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use petgraph::unionfind::UnionFind;

extern crate regex;
use regex::Regex;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut chars = s.chars().map(|c| c.to_string()).collect_vec();
    chars.reverse();

    let mut queue = VecDeque::new();
    queue.push_front(n);
    for (i, char) in chars.iter().enumerate() {
        if char == "R" {
            queue.push_front(n - i - 1);
        } else {
            queue.push_back(n - i - 1);
        }
    }

    println!("{}", queue.iter().join(" "));
}
