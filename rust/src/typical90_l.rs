#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use petgraph::unionfind::UnionFind;

fn get_idx(i: usize, j: usize, w: usize) -> usize {
    return i * w + j;
}
fn get_four(i: usize, j: usize) -> Vec<(usize, usize)> {
    return vec![
        (i - 1, j    ),
        (i    , j - 1),
        (i + 1, j    ),
        (i    , j + 1),
    ];
}

fn main() {
    input! {
        h: usize, w: usize,
        n: usize
    }

    let mut used = vec![vec![false; w + 2]; h + 2];
    let x = (h + 2) * (w + 2);
    let mut uf = UnionFind::<usize>::new(x);
    let mut ans = vec![];

    for _ in 0..n {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                r: usize, c: usize,
            }
            if !used[r][c] {
                used[r][c] = true;
                for (i, j) in get_four(r, c) {
                    if used[i][j] {
                        let idx1 = get_idx(r, c, w);
                        let idx2 = get_idx(i, j, w);
                        uf.union(idx1, idx2);
                    }
                }
            }
        } else {
            input! {
                i: usize, j: usize, s: usize, t: usize,
            }
            let idx1 = get_idx(i, j, w);
            let idx2 = get_idx(s, t, w);
            if i == s && j == t {
                if used[i][j] {
                    ans.push("Yes");
                } else {
                    ans.push("No");
                }
            } else if uf.equiv(idx1, idx2) {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }
    for s in ans {
        println!("{}", s);
    }
}
