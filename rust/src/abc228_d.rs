#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BTreeSet;
use std::collections::HashMap;

const MOD: usize = 1048576;

fn main() {
    input! {
        q: usize,
        queries: [(u8, usize); q],
    }

    // <_> と書いておくと型推論可能な範囲で解決してくれる
    let mut set = (0..=MOD).collect::<BTreeSet<_>>();
    let mut map = HashMap::new();

    for &(t, x) in queries.iter() {
        match t {
            1 => {
                let h = match *set.range((x % MOD)..).next().unwrap() {
                    MOD => *set.range(0..).next().unwrap(),
                    resp => resp,
                };

                set.remove(&h);
                map.insert(h, x);
            },
            2 => {
                match map.get(&(x % MOD)) {
                    Some(&ans) => println!("{}", ans),
                    None => println!("{}", -1),
                }
            },
            _ => (),
        }
    }
}
