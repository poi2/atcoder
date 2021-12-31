#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;

    let mut l = 0;
    for r in 0..n {
        *map.entry(an[r]).or_insert(0) += 1;

        while map.len() > k {
            let sub = map.get(&an[l]).unwrap();
            if *sub == 1 {
                map.remove(&an[l]);
            } else {
                map.insert(an[l], sub - 1);
            }
            l += 1;
        }

        ans = ans.max(r - l + 1);
    }
    println!("{}", ans);
}
