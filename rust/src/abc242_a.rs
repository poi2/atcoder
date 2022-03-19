use proconio::input;

fn main() {
    input! {
        a: usize, b: usize, c: usize, x: usize,
    }
    if x <= a {
        println!("{}", 1);
    } else if b < x {
        println!("{}", 0);
    } else {
        let ans = (c as f64) / ((b - a) as f64);
        println!("{}", ans);
    }
}
