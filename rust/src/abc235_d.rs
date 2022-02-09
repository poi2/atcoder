#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        a: usize, n: usize,
    }
    let max = std::u128::MAX;
    let mut queue = VecDeque::new();
    let mut used = vec![max; n * 10];

    queue.push_back((n, 0));
    let mut ans = max;

    while let Some((num, dist)) = queue.pop_front() {
        if used[num] != max {
            continue;
        }
        used[num] = dist;

        if num == 1 {
            ans = ans.min(dist);
        }

        if num % a == 0 {
            queue.push_back((num / a, dist + 1));
        }
        if num >= 10 && num % 10 != 0 {
            if let Some(rotated) = num_rotate(num) {
                queue.push_back((rotated, dist + 1));
            }
        }
    }

    if ans == max {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

fn num_rotate(num: usize) -> Option<usize> {
    let mut list = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<usize>>();

    list.rotate_left(1);
    if *list.get(0).unwrap() == 0 {
        return None;
    }

    let ans = list
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    return Some(ans);
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(num_rotate(72_usize), Some(27_usize));
    assert_eq!(num_rotate(720_usize), Some(207_usize));
    assert_eq!(num_rotate(702_usize), None);
}
