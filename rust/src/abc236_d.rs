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
    // let mut used = vec![false; 2 * n];
    // let ans = old_solve(0, 0, 0, &mut used, n, &chems);
    let ans = solve(0,  0, 0, n, &chems);

    println!("{}", ans);
}

fn to_bit(i: usize) -> usize { 1 << i }
fn is_used(used: usize, i: usize) -> bool {
    let bit = to_bit(i);
    used & bit == bit
}
fn set_used(used: usize, i: usize, j: usize) -> usize { used | to_bit(i) | to_bit(j) }

fn solve(i: usize, acc: usize, used: usize ,max_team: usize, chems: &Vec<Vec<usize>>) -> usize {
    if used == to_bit(2 * max_team) - 1 {
        return acc;
    }

    if is_used(used, i) {
        return solve(i + 1, acc, used, max_team, chems);
    }
    let mut max = 0;
    for j in (i + 1)..(max_team * 2) {
        if !is_used(used, j) {
            let next_acc = acc ^ chems[i][j - i - 1];
            let next_used = set_used(used, i, j);
            let ret = solve(i + 1, next_acc, next_used, max_team, chems);
            max = max.max(ret);
        }
    }
    return max;
}

// fn old_solve(i: usize, team_count: usize, acc: usize, used: &mut Vec<bool> ,max_team: usize, chems: &Vec<Vec<usize>>) -> usize {
//     if team_count == max_team {
//         return acc;
//     }

//     if used[i] == true {
//         return old_solve(i + 1, team_count, acc, used, max_team, chems);
//     } else {
//         used[i] = true;
//         let mut max = 0;
//         for j in (i + 1)..(max_team * 2) {
//             if used[j] == false {
//                 used[j] = true;
//                 let ret = old_solve(
//                     i + 1,
//                     team_count + 1,
//                     acc ^ chems[i][j - i - 1],
//                     used,
//                     max_team,
//                     chems
//                 );
//                 max = max.max(ret);
//                 used[j] = false
//             }
//         }
//         used[i] = false;
//         return max;
//     }
// }
