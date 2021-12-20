use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        s: String,
    };
    let list = s.split("x").collect_vec();
    println!("{}", list[0].parse::<i32>().unwrap() * list[1].parse::<i32>().unwrap());
}
