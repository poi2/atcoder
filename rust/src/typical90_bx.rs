#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::f32::consts::E;

use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }

    let expected = an.iter().sum::<usize>() / 10;

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    if expected > 0 {
        while sum != expected && i < n {
            if sum < expected {
                sum += an[j];
                j += 1;
                j %= n;
            } else {
                sum -= an[i];
                i += 1;
            }
        }
    }

    if sum == expected && expected != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
