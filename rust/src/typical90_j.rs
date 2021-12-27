// https://atcoder.jp/contests/typical90/tasks/typical90_j

use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut cum_sum_a: Vec<usize> = vec![0; n + 1];
    let mut cum_sum_b: Vec<usize> = vec![0; n + 1];

    for i in 0..n {
        let (a, b) = if cp[i].0 == 1 {
            (cp[i].1, 0)
        } else {
            (0, cp[i].1)
        };

        cum_sum_a[i + 1] = cum_sum_a[i] + a;
        cum_sum_b[i + 1] = cum_sum_b[i] + b;
    }

    for i in 0..q {
        let l = lr[i].0 - 1;
        let r = lr[i].1;

        let ans_a = cum_sum_a[r] - cum_sum_a[l];
        let ans_b = cum_sum_b[r] - cum_sum_b[l];
        println!("{} {}", ans_a, ans_b);
    }
}
