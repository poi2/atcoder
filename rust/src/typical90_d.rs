// https://atcoder.jp/contests/typical90/tasks/typical90_d

use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut lines = vec![0; h];
    let mut colums = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            lines[i] += a[i][j];
            colums[j] += a[i][j];
        }
    }

    for i in 0..h {
        let mut ans = vec![0; w];
        for j in 0..w {
            ans[j] = lines[i] + colums[j] - a[i][j];
        }
        println!("{}", ans.iter().map(|x| x.to_string()).join(" ") );
    }
}
