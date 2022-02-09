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

use petgraph::{graph::DiGraph, graph::NodeIndex, algo::dijkstra};

fn main() {
    input! {
        n: usize, m: usize,
        hn: [usize; n],
        uvm: [(usize, usize); m],
    }

    let edges = uvm
        .iter()
        .map(|(u, v)| (u - 1, v - 1) )
        .fold(vec![], |mut vec, (u, v)| {
            vec.push((u, v, hn[v].saturating_sub(hn[u])));
            vec.push((v, u, hn[u].saturating_sub(hn[v])));
            return vec;
        });

    let graph = DiGraph::<usize, usize, usize>::from_edges(edges.iter());
    let resp = dijkstra(&graph, NodeIndex::new(0), None, |er| *er.weight());

    let ans = (1..n)
        .map(|i| hn[0].saturating_sub(hn[i]).saturating_sub(resp[&NodeIndex::new(i)]))
        .max()
        .unwrap();

    println!("{}", ans);
}
