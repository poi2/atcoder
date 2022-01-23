#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, m: usize,
        sn: [String; n],
        tm : [String; m],
    }
    let mut stops: HashMap<String, bool> = HashMap::new();
    for t in tm {
        stops.insert(t, true);
    }
    for s in sn {
        let ans = if stops.contains_key(&s) {
            "Yes"
        } else {
            "No"
        };
        println!("{}", ans);
    }
}