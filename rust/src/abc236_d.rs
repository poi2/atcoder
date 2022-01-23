#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
    }
    let mut chems = vec![];
    for i in 0..2*n {
        input! { x: [usize; 2 * n - 1 - i] }
        chems.push(x);
    }
    let mut used = vec![false; 2 * n];
    let ans = solve(0, 0, 0, &mut used, n, &chems);

    println!("{}", ans);
}

fn solve(i: usize, team_count: usize, acc: usize, used: &mut Vec<bool> ,max_team: usize, chems: &Vec<Vec<usize>>) -> usize {
    if team_count == max_team {
        return acc;
    }

    if used[i] == true {
        return solve(i + 1, team_count, acc, used, max_team, chems);
    } else {
        used[i] = true;
        let mut max = 0;
        for j in (i + 1)..(max_team * 2) {
            if used[j] == false {
                used[j] = true;
                let ret = solve(
                    i + 1,
                    team_count + 1,
                    acc ^ chems[i][j - i - 1],
                    used,
                    max_team,
                    chems
                );
                max = max.max(ret);
                used[j] = false
            }
        }
        used[i] = false;
        return max;
    }
}
