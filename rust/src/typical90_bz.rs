#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, m: usize,
        mut an: [[usize; 2]; m],
    }
    let mut counter = HashMap::new();
    for v in an {
        let x = v[0];
        let y = v[1];
        if x > y {
            if let Some(n) = counter.get_mut(&x) {
                *n += 1;
            } else {
                counter.insert(x, 1);
            }
        } else {
            if let Some(n) = counter.get_mut(&y) {
                *n += 1;
            } else {
                counter.insert(y, 1);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        if let Some(n) = counter.get(&i) {
            if *n == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
