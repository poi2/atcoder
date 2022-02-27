#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, m: usize,
        an: [usize; n],
        bm: [usize; m],
    }
    let mut stock: HashMap<usize, usize> = HashMap::new();
    for a in an {
        match stock.get_mut(&a) {
            Some(v) => { *v += 1; },
            None => { stock.insert(a, 1); },
        }
    }
    let mut ans = "Yes";
    for b in bm {
        match stock.get_mut(&b) {
            Some(v) => {
                if *v == 0 {
                    ans = "No";
                    break;
                } else {
                    *v -= 1;
                };
            },
            None => { ans = "No"; },
        }
    }
    println!("{}", ans);
}
