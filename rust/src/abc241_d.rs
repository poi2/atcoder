#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: usize,
    }
    let mut btree_set = BTreeSet::new();
    let mut ans = vec![];
    for i in 0..q {
        input! { c: usize }
        match c {
            1 => {
                input! { x: i128 }
                btree_set.insert((x, i));
            },
            2 => {
                input! { x: i128, k: usize }
                if let Some(&(a, _i)) = btree_set.range(..(x + 1, 0)).nth_back(k - 1) {
                    ans.push(a);
                } else {
                    ans.push(-1);
                }
            },
            3 => {
                input! { x: i128, k: usize }
                if let Some(&(a, _i)) = btree_set.range((x, 0)..).nth(k - 1) {
                    ans.push(a);
                } else {
                    ans.push(-1);
                }
            },
            _ => (),
        }
    }
    for a in ans { println!("{}", a)};
}
