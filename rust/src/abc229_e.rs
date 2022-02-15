#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize, m: usize,
        abm: [(usize, usize); m],
    }

    let mut vec = vec![vec![]; n];
    for (aa, bb) in abm {
        let (a, b) = if aa < bb { (aa, bb) } else { (bb, aa) };
        vec[a - 1].push(b - 1);
    }

    let mut uf = UnionFind::<usize>::new(n + 1);
    let mut ans_list = vec![0];
    let mut size = 0;
    for i in (1..n).map(|ii| n - ii ) {
        size += 1;
        for &j in &vec[i] {
            if !uf.equiv(i, j) {
                size -= 1;
            }
            uf.union(i, j);
        }
        ans_list.push(size);
    }

    while let Some(ans) = ans_list.pop() {
        println!("{}", ans);
    }
}
