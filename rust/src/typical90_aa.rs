#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        if !map.contains_key(&sn[i]) {
            println!("{}", i + 1);
            map.insert(&sn[i], true);
        }
    }
}
