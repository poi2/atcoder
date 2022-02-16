#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
    }

    let mut heads = HashMap::new();
    let mut pairs = Vec::new();
    let mut lines = vec![];
    for i in 0..m {
        input! {
            k: usize,
            ak: [usize; k],
        }
        let h = ak[0];
        if let Some(&j) = heads.get(&h) {
            pairs.push((i, j));
        } else {
            heads.insert(ak[0], i);
        }
        lines.insert(i, VecDeque::from(ak));
    }

    let mut count = 0;
    while let Some((i, j)) = pairs.pop() {
        count += 1;
        lines[i].pop_front();
        lines[j].pop_front();
        if let Some(s) = lines[i].front() {
            if let Some(t) = heads.get(s) {
                pairs.push((i, *t));
            } else {
                heads.insert(*s, i);
            }
        }
        if let Some(s) = lines[j].front() {
            if let Some(t) = heads.get(s) {
                pairs.push((j, *t));
            } else {
                heads.insert(*s, j);
            }
        }
    }

    println!("{}", if count == n { "Yes" } else { "No" });
}
