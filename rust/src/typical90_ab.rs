#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        poss: [(usize, usize, usize, usize); n],
    }
    let max_x = poss.iter().map(|(x1, _, x2, _)| x1.max(x2)).max().unwrap() + 1;
    let max_y = poss.iter().map(|(_, y1, _, y2)| y1.max(y2)).max().unwrap() + 1;

    let mut table: Vec<Vec<i64>> = vec![vec![0; max_y]; max_x];
    for (x1, y1, x2, y2) in poss {
        table[x1][y1] += 1;
        table[x1][y2] -= 1;
        table[x2][y1] -= 1;
        table[x2][y2] += 1;
    }

    for i in 0..max_x {
        let mut count: i64 = 0;
        for j in 0..max_y {
            count += table[i][j];
            table[i][j] = count;
        }
    }

    let mut ans_list: Vec<usize> = vec![0; n + 1];

    for j in 0..max_y {
        let mut count: i64 = 0;
        for i in 0..max_x {
            count += table[i][j];
            table[i][j] = count;
            ans_list[count as usize] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", ans_list[i]);
    }
}
