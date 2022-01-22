#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, p: usize, q: usize,
        an: [usize; n],
    }
    // let ans = an.iter()
    //     .combinations(5)
    //     .map(|combis| combis.iter().fold(1, |acc, &&i| acc * i ) )
    //     .filter(|ret| ret % p == q)
    //     .count();
    // let ans = (0..n).combinations(5)
    //     .map(|ids| ids.iter().fold(1, |acc, &i| (acc * an[i]) % p ) )
    //     .filter(|&ret| ret == q)
    //     .count();
    let ans = dfs(0, 0, 1, &an, n, p, q);
    println!("{}", ans);
}

fn dfs(depth: usize, i: usize, acc: usize, an: &Vec<usize>, n: usize, p: usize, q: usize) -> usize {
    if depth == 5 {
        return if acc % p == q { 1 } else { 0 };
    }

    // let mut ans = 0;
    // for j in i..n {
    //     ans += dfs(depth + 1, j + 1, acc * an[j] % p, an, n, p, q);
    // }
    let ans = (i..n)
        .map(|j| dfs(depth + 1, j + 1, acc * an[j] % p, an, n, p, q))
        .sum();
    return ans;
}
