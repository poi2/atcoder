// https://atcoder.jp/contests/abc222/tasks/abc222_d

use proconio::input;

fn main() {
    input! {
        str: String,
    }
    let mut chars: Vec<String> = str.chars().map(|c| c.to_string()).collect();
    chars.sort();
    chars.dedup();

    let ans = match chars.len() {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => 0
    };
    println!("{}", ans);
}
