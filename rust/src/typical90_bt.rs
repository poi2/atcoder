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
        h: usize, w: usize,
        chw: [String; h],
    }
    let mut map = vec![vec![false; w + 2]; h + 2];
    for i in 0..h {
        let line = chw[i].chars().collect_vec();
        for j in 0..w {
            map[i + 1][j + 1] = line[j] == '.';
        }
    }
    // println!("{:?}", map);

    let mut ans = 0;
    let mut used = vec![vec![false; w + 2]; h + 2];
    for i in 1..h+1 {
        for j in 1..w+1 {
            if let Some(a) = dfs(i, j, 0, 0, &map, &mut used, i, j) {
                ans = ans.max(a);
            }
        }
    }
    if 3 <= ans {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }

}

fn dfs(x: usize, y: usize, dist: usize, mut ans: usize, map: &Vec<Vec<bool>>, used: &mut Vec<Vec<bool>>, goal_x: usize, goal_y: usize) -> Option<usize> {
    if !map[x][y] { return None; }
    if (x, y) == (goal_x, goal_y) && dist > 0 { return Some(dist); }
    if used[x][y] { return None; }
    used[x][y] = true;

    for (next_x, next_y) in neighbours(x, y) {
        if !map[next_x][next_y] { continue; }
        if let Some(a) = dfs(next_x, next_y, dist + 1, ans, map, used, goal_x, goal_y) {
            ans = ans.max(a);
        }
    }
    used[x][y] = false;
    return Some(ans);
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    return vec![
        (x - 1, y    ),
        (x + 1, y    ),
        (x    , y - 1),
        (x    , y + 1),
    ];
}

