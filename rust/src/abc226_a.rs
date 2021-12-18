use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }
    let mut an = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            al: [usize; l],
        }
        let a = al.iter().map(|i| i.to_string()).collect_vec().join(" ");
        an.push(a);
    }
    let uniq: HashSet<_> = an.into_iter().collect();
    println!("{}", uniq.len());
}
