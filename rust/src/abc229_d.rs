#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize,
    }
    let list: Vec<bool> = s
        .chars()
        .map(|c| c == 'X')
        .collect();

    let ans =
        if k == 0 {
            max_chain_size(list)
        } else {
            two_pointer_algo(list, k)
        };

    println!("{}", ans);
}

fn max_chain_size(list: Vec<bool>) -> usize {
    let (ans, _chain) = list
        .iter()
        .fold((0, 0), |(max, chain), &bool| {
            if bool {
                (max.max(chain + 1), chain + 1)
            } else {
                (max, 0)
            }
        });
    return ans;
}

// NOTE イレギュラーな要素が margin 個以内なら許される尺取法
fn two_pointer_algo(list: Vec<bool>, margin: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let n = list.len();

    let mut max_size = 0;
    let mut chain = 0;
    let mut cur_margin = 0;

    while i < n {
        while j < n && (cur_margin < margin || cur_margin == margin && list[j]) {
            if !list[j] { cur_margin += 1; }
            chain += 1;
            j += 1;
        }
        max_size = max_size.max(chain);

        if !list[i] { cur_margin -= 1; }
        i += 1;
        chain -= 1;
    }

    return max_size;
}
