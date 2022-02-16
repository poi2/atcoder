#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize, q: usize,
        qs: [(u8, usize); q],
    }
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(l);
    for (c, x) in qs {
        match c {
            1 => {
                set.insert(x);
            },
            2 => {
                let min = set.range(..x).last().unwrap();
                let max = set.range(x..).next().unwrap();
                println!("{}", max - min);
            },
            _ => (),
        }
    }
}
