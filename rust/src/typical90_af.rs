#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use itertools::enumerate;

fn is_good_order(order: Vec<usize>, bads: Vec<Vec<usize>>) -> bool {
    for i in 0..order.len() - 1 {
        if bads[order[i]].contains(&order[i + 1]) {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        n: usize,
        aij: [[usize; n]; n],
        m: usize,
        xyn: [(usize, usize); m],
    }
    let mut bads = vec![vec![]; n + 1];
    for (x, y) in xyn {
        bads[x-1].push(y-1);
        bads[y-1].push(x-1);
    }

    let good_orders = (0..n)
        .permutations(n)
        .filter(|order|
            is_good_order(order.to_vec(), bads.to_vec())
        );


    let mut min = std::i32::MAX as usize;
    for order in good_orders {
        let sum: usize = order
            .iter()
            .enumerate()
            .map(|(j, &i)| aij[i][j] )
            .sum();
        min = min.min(sum);
    }

    if min == std::i32::MAX as usize {
        println!("{}", -1);
    } else {
        println!("{}", min);
    }
}
