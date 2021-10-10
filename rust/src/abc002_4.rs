// https://atcoder.jp/contests/abc002/tasks/abc002_4
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &(x, y) in &xy {
        // x
        if map.contains_key(&x) {
            map.get_mut(&x).unwrap().insert(y);
        } else {
            let mut set = HashSet::new();
            set.insert(y);
            map.insert(x, set);
        }
        // y
        if map.contains_key(&y) {
            map.get_mut(&y).unwrap().insert(x);
        } else {
            let mut set = HashSet::new();
            set.insert(x);
            map.insert(y, set);
        }
    }
    /*
    あとは数え上げる
    https://atcoder.jp/contests/abc002/submissions/24150851
    */
}
