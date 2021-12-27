use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        bnm: [[usize; m]; n],
    }

    let center = bnm[0][0];
    let mut ans = true;
    let line = (center - 1) / 7;
    for i in 0..n {
        for j in 0..m {
            if bnm[i][j] != center + (i * 7) + j || (line + i) != (bnm[i][j] - 1) / 7 {
                ans = false;
            }
        }
    }
    let ans_str = if ans == true { "Yes" } else { "No" };
    println!("{}", ans_str);
}
