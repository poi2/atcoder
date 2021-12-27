// https://atcoder.jp/contests/abc218/tasks/abc218_d

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        points: [(usize, usize); n]
    }
    let mut map = HashMap::new();
    for point in points.iter() {
        map.insert(point, true);
    }
    let mut ans: usize = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !(points[i].0 == points[j].0 || points[i].1 == points[j].1) {
                let p3 = (points[i].0, points[j].1);
                let p4 = (points[j].0, points[i].1);
                if map.contains_key(&p3) && map.contains_key(&p4) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans / 2);
}
