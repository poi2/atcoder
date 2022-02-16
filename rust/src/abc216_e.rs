#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize, k: usize,
        mut an: [usize; n],
    }
    an.sort();
    an.reverse();
    an.push(0);

    let mut i = 0;
    let mut fun = 0;
    let mut count = 0;
    while i < n && count < k {
        let fan0 = an[i];
        let fan1 = an[i + 1];
        let n = fan0 - fan1;

        if count + (i + 1) * n < k {
            count += (i + 1) * n;
            fun += n * (fan0 + fan1 + 1) / 2 * (i + 1);
        } else {
            let rem = k - count;
            count = k;
            let n = rem / (i + 1);
            let r = rem % (i + 1);
            let fan1 = fan0 - n;
            fun += n * (fan0 + fan1 + 1) / 2 * (i + 1);
            fun += fan1 * r;
        }
        i += 1;
    }
    println!("{}", fun);
}
