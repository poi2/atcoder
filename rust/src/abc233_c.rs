#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use itertools::iproduct;

// fn dfs(i: usize, prod: usize, goal: usize, bag_size: usize, aij: &Vec<Vec<usize>>, ans: &mut usize) {
//     if i == bag_size {
//         if prod == goal {
//             *ans += 1;
//         }
//         return;
//     }
//     for j in &aij[i] {
//         if prod <= goal / j {
//             dfs(i + 1, prod * j, goal, bag_size, aij, ans);
//         }
//     }
// }

fn main() {
    // input! {
    //     bag_size: usize,
    //     goal: usize,
    // }
    // let mut aij: Vec<Vec<usize>> = vec![vec![]; bag_size];

    // for i in 0..bag_size {
    //     input! {
    //         l: usize,
    //         an: [usize; l],
    //     }
    //     aij[i] = an;
    // }
    // let mut ans = 0;

    // dfs(0, 1, goal, bag_size, &aij, &mut ans);
    // println!("{}", ans);

    input! {
        bag_size: usize,
        goal: u128,
    }
    let mut aij= vec![];

    for _ in 0..bag_size {
        input! {
            l: u128,
            an: [u128; l],
        }
        aij.push(an);
    }

    let ans = aij
        .iter()
        .multi_cartesian_product()
        .map(|prod_vec| prod_vec.iter().fold(1, |acc, &&u| acc * u))
        .filter(|&n| n == goal)
        .count();
    println!("{}", ans);
}
