#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    println!("{}", solve(n, s));
}

fn solve(n: usize, s: String) -> usize {
    let mut blocks = vec![];
    let chars = s.chars().collect_vec();
    let mut past_item = chars[0];
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == past_item {
            count += 1;
        } else {
            blocks.push(count);
            past_item = chars[i];
            count = 1;
        }
    }
    blocks.push(count);

    if blocks.len() == 1 {
        return 0;
    }

    let all_pattern = n * (n - 1) / 2;
    let irregular_pattern = blocks
        .iter()
        .map(|&n| n * (n - 1) / 2)
        .sum::<usize>();
    return all_pattern - irregular_pattern;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = vec![
            (4, "ooxo", 5),
            (5, "oxoxo", 10),
            (5, "ooooo", 0),
            (7, "xxoooxx", 16)
        ];
        for &(n, s, ans) in list.iter() {
            assert_eq!(solve(n, s.to_string()), ans);
        }
    }
}

