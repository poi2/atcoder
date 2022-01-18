#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
        cn: [usize; n],
    }
    let div = 46;
    let mut zip = vec![vec![0; div]; 3];

    for a in an { zip[0][a % div] += 1; }
    for b in bn { zip[1][b % div] += 1; }
    for c in cn { zip[2][c % div] += 1; }

    let mut ans: usize = 0;
    for i in 0..div {
        for j in 0..div {
            let k = (3 * div - (i + j)) % div;
            ans += zip[0][i] * zip[1][j] * zip[2][k];
        }
    }
    println!("{}", ans)
}
