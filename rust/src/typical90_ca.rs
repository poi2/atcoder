#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        mut ahw: [[isize; w]; h],
        bhw: [[isize; w]; h],
    }

    let mut count = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let a = ahw[i][j];
            let b = bhw[i][j];
            let diff = b - a;
            if diff != 0 {
                ahw[i    ][j    ] += diff;
                ahw[i + 1][j    ] += diff;
                ahw[i    ][j + 1] += diff;
                ahw[i + 1][j + 1] += diff;

                count += diff.abs();
            }
        }
    }
    let mut ans = "Yes";
    for i in 0..h {
        if ahw[i][w - 1] != bhw[i][w - 1] {
            ans = "No";
        }
    }
    for j in 0..w {
        if ahw[h - 1][j] != bhw[h - 1][j] {
            ans = "No";
        }
    }
    println!("{}", ans);
    if ans == "Yes" {
        println!("{}", count);
    }
}
