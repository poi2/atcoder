#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let mut vec: Vec<(usize, usize)> = vec![];
    let mut depth = 0;
    for a in an {
        if let Some((num, count)) = vec.pop() {
            if num == a {
                if count + 1 == num {
                    depth -= count;
                } else {
                    vec.push((num, count + 1));
                    depth += 1;
                }
            } else {
                vec.push((num, count));
                vec.push((a, 1));
                depth += 1;
            }
        } else {
            vec.push((a, 1));
            depth += 1;
        }
        println!("{}", depth);
    }
}
