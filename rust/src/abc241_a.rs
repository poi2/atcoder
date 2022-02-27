#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        an: [usize; 10],
    }
    let mut id = 0;
    for _ in 0..3 {
        id = an[id];
    }
    println!("{}", id);
}
