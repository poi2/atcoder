#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        t: usize,
        asn: [(usize, usize); t],
    }
    for (a, s) in asn {
        let mut ans = "No";
        if s >= 2 * a {
            let b = s - 2 * a;
            if a & b == 0  {
                ans = "Yes";
            }
        };
        println!("{}", ans);
    }
}

/*
x AND y = a
x XOR y = b
と置くと、
s = 2 * a + b
とできる。
また a AND b = 0 となる（AND と XOR は全ビット 1,0, or 0,1 になる）。

b > 0 という前提から a は s = 2 * a + b から s >= 2 * a でなければならない。
s = 2 * a + b から b = s - 2 * a となる。
また、 a AND b = 0 でなければならない。
*/
