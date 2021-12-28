use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let ans = ((y - x).max(0) + 9) / 10;
    println!("{}", ans);
}
