#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aij: [[usize; 6]; n],
    }
    let div = 10_usize.pow(9) + 7;

    // NOTE TLE
    // let ans = aij.iter()
    //     .multi_cartesian_product()
    //     .map(|vec| vec.iter().fold(1, |acc, &&a| acc * a))
    //     .fold(0, |acc, i| acc + i) % div;
    // println!("{}", ans);

    // let mut ans = 1;
    // for vec in aij {
    //     let add = vec.iter().fold(0, |acc, a| acc + a ) % div;
    //     ans = (ans * add) % div;
    // }

    let ans = aij.iter()
        .map(|vec| vec.iter().sum() )
        .fold(1, |acc, add: usize| acc * add % div);

    println!("{}", ans);
}
