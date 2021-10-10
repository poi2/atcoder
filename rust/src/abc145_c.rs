// https://atcoder.jp/contests/abc145/tasks/abc145_c

use proconio::input;
use itertools::Itertools;

fn euclid_dist(s: (f64, f64), d: (f64, f64)) -> f64 {
    let inner = (s.0 - d.0).powi(2) + (s.1 - d.1).powi(2);
    inner.sqrt()
}

fn main() {
    input! {
        n: usize,
        positions: [(f64, f64); n]
    }
    let mut dist: f64 = 0.0;
    let mut count: f64 = 0.0;
    for p in (0..n).permutations(n){
        count += 1.0;

        for i in 0..n-1 {
            let s = positions[p[i]];
            let d = positions[p[i+1]];
            dist += euclid_dist(s, d);
        }
    }
    println!("{}", dist / count);
}
