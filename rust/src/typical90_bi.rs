use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut vec = VecDeque::new();
    for (t, x) in queries {
        match t {
            1 => vec.push_front(x),
            2 => vec.push_back(x),
            3 => println!("{}", vec[x - 1]),
            _ => (),
        }
    }
}