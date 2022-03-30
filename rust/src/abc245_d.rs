use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [isize; n + 1],
        c: [isize; n + m + 1],
    }
    let mut b = vec![0; n + m + 1];
    for i in (0..n + m + 1).rev().take(m + 1) {
        let bi = i - n.min(i);
        let ai = i - bi;
        let mut sub = 0;
        for j in 0..n {
            sub += b[i - j] * a[j];
        }
        b[bi] = (c[i] - sub) / a[ai];
    }
    let ans = b.iter().take(m + 1).map(|i| *i).collect::<Vec<_>>();
    println!("{}", ans.iter().join(" "));
}
