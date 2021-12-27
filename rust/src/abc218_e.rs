// https://atcoder.jp/contests/abc218/tasks/abc218_e

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut list: [(usize, usize, i64); m],
    }

    list.sort_unstable_by_key(|abc| abc.2);

    let mut tree = UnionFind::new(n);
    let mut ans = 0;
    for (x, y, cost) in list {
        if !tree.union(x - 1, y - 1) && cost > 0 {
            ans += cost;
        }
    }
    println!("{}", ans);
}
