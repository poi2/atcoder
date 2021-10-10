// https://atcoder.jp/contests/abc170/tasks/abc170_b
use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32
    }
    let mut i = 0;
    let mut ans = false;
    while i <= x && ans == false {
        if 4 * i + 2 * (x - i) == y {
            ans = true;
            break;
        }
        i += 1;
    }
    if ans == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
