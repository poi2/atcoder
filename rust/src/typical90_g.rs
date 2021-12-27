#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
        q: usize,
        bq: [usize; q],
    }
    let mut list = BTreeSet::new();
    for a in an {
        list.insert(a);
    }

    for b in bq.iter() {
        let ox = list.range(..=b).last();
        let oy = list.range(b..).next();
        match (ox, oy) {
            (None, None) => println!(""),
            (None, Some(y)) => println!("{}", (y - b)),
            (Some(x), None) => println!("{}", (b - x)),
            (Some(x), Some(y)) => {
                let ans1 = b - x;
                let ans2 = y - b;
                println!("{}", ans1.min(ans2));
            }
        }
    }
}
