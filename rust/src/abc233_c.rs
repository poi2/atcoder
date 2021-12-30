#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::iproduct;

fn dfs(i: usize, prod: usize, goal: usize, bag_size: usize, aij: &Vec<Vec<usize>>, ans: &mut usize) {
    if i == bag_size {
        if prod == goal {
            *ans += 1;
        }
        return;
    }
    for j in &aij[i] {
        if prod <= goal / j {
            dfs(i + 1, prod * j, goal, bag_size, aij, ans);
        }
    }
}

fn main() {
    input! {
        bag_size: usize,
        goal: usize,
    }
    let mut aij: Vec<Vec<usize>> = vec![vec![]; bag_size];

    for i in 0..bag_size {
        input! {
            l: usize,
            an: [usize; l],
        }
        aij[i] = an;
    }
    let mut ans = 0;

    dfs(0, 1, goal, bag_size, &aij, &mut ans);
    println!("{}", ans);
}
