use proconio::input;

fn main() {
    input! {
        s: usize,
    }

    let div = 10_usize.pow(9) + 7;
    let mut dp = vec![0; s + 1];
    for i in 3..=s {
        let sum = (3..=i-3)
            .into_iter()
            .fold(0, |sum, j| sum + dp[j]);
        dp[i] = (sum + 1) % div;
    }
    println!("{}", dp[s])
}
