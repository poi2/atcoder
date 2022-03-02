#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let max = 200_001;
    let mut tree = UnionFind::new(max);
    let mut ans = 0;
    for i in 0..n/2 {
        let j = n - 1 - i;
        if an[i] != an[j] && tree.union(an[i], an[j]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
