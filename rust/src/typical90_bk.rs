#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize, w: usize,
        pij: [[usize; w]; h],
    }
    let mut ans = 0;
    for i in 1..1<<h {
        let is = gen(i);
        let tmp = max_size(&is, &pij, w);
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}

fn gen(mut i: usize) -> Vec<usize> {
    let mut vec = vec![];
    while i > 0 {
        vec.push(i & 1);
        i = i >> 1;
    }
    let ret: Vec<usize> = vec
        .iter()
        .enumerate()
        .filter(|&(_i, v)| *v == 1)
        .map(|(i, _v)| i)
        .collect();
    return ret;
}

fn max_size(is: &Vec<usize>, pij: &Vec<Vec<usize>>, w: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for j in 0..w {
        let head = pij[is[0]][j];
        let bool = is.iter()
            .fold(true, |acc, &i| acc && (pij[i][j] == head));

        if bool {
            if let Some(x) = map.get_mut(&head) {
                *x += 1;
            } else {
                map.insert(head, 1);
            }
        }
    }

    match map.values().max() {
        Some(&ans) => return ans * is.len(),
        None => return 0,
    }
}
