#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use petgraph::unionfind::UnionFind;

/*
UnionFind の union を破壊的変更するように変更してみた
外部のクレートには外側から変更を加えることができないので、
エイリアス型を作って、それに trait で I/F を作り、実装を追加した。
type MyUnionFind = UnionFind<usize>;
trait MyUnionFindDestructiveMethod {
    fn union_and_self(self, x: usize, y: usize) -> Self;
}
impl MyUnionFindDestructiveMethod for MyUnionFind {
    fn union_and_self(mut self, x: usize, y: usize) -> Self {
        self.union(x, y);
        return self;
    }
}
fn main() {
    input! {
        n: usize, q: usize,
        lrq: [(usize, usize); q],
    }
    let bool = lrq
        .iter()
        .fold(MyUnionFind::new(n + 1), |tree, &(l, r)| tree.union_and_self(l - 1, r))
        .equiv(0, n);
    println!("{}", if bool { "Yes" } else { "No" });
}
*/

fn main() {
    input! {
        n: usize, q: usize,
        lrq: [(usize, usize); q],
    }
    let bool = lrq
        .iter()
        .fold(UnionFind::new(n + 1), |mut tree, &(l, r)| {
            tree.union(l - 1, r);
            return tree;
        })
        .equiv(0, n);

    println!("{}", if bool { "Yes" } else { "No" });
}
