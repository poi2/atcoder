// https://atcoder.jp/contests/abc138/tasks/abc138_d

use itertools::Itertools;
use proconio::input;

fn dfs(idx: usize, pre_idx: usize, tree: &Vec<Vec<usize>>, cost: &mut Vec<usize>, used: &mut Vec<bool>) {
    used[idx] = true;
    if pre_idx != 2147483648 {
        cost[idx] += cost[pre_idx];
    }
    for next_idx in &tree[idx] {
        if !used[*next_idx] {
            dfs(*next_idx, idx, tree, cost, used);
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize
    }

    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        tree[a-1].push(b-1);
        tree[b-1].push(a-1);
        // 双方向で繋がっていて、a, b のどちらが木の根側なのかは分からない
        // なので、双方向に繋がっているという情報を持ち、検索済みの葉は再計算しないようにする
    }
    // println!("{:?}", tree);
    let mut cost = vec![0; n];
    for _ in 0..q {
        input! {
            p: usize,
            x: usize,
        }
        cost[p-1] += x;
    }

    let mut used = vec![false; n];
    dfs(0, 2147483648, &tree, &mut cost, &mut used);
    println!("{}", cost.iter().map(|x| x.to_string()).join(" "));
}
