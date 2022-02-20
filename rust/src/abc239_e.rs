#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize, q: usize,
        xn: [usize; n],
        abn: [(usize, usize); n - 1],
        vkq: [(usize, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in abn {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut memo = vec![vec![]; n];
    dfs(&graph, &xn, &mut memo, 0, 0);

    for (v, k) in vkq {
        println!("{}", memo[v - 1][k - 1]);
    }
}

fn dfs(graph: &Vec<Vec<usize>>, xn: &Vec<usize>, memo: &mut Vec<Vec<usize>>, i: usize, parent: usize) -> Vec<usize> {
    let mut order = vec![xn[i]];
    for &child in &graph[i] {
        if child == parent { continue; }
        order.append(&mut dfs(graph, xn, memo, child, i));
    }

    order.sort();
    order.reverse();
    order.resize(20, 0);
    memo[i] = order.clone();
    return order;
}
