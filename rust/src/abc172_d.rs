use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 1..=n {
        let len = n / i;
        ans += (i + len * i) * len / 2;
    }
    println!("{}", ans);
}
