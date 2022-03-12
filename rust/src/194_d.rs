use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let n_f64 = n as f64;
    let ans = (1..n)
        .map(|i| i as f64)
        .collect::<Vec<f64>>()
        .iter()
        .fold(0_f64, |acc, &i| acc + n_f64 / i);
    println!("{}", ans);
}
