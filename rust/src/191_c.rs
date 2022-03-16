use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        string_list: [String; h],
    }
    let map = string_list
        .iter()
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|&c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<usize>>()
        }).collect_vec();

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let neig_cnt = get4(i, j)
                .iter()
                .map(|&(s, t)| map[s][t] )
                .sum::<usize>();
            ans += match neig_cnt {
                1 => 1,
                3 => 1,
                _ => 0,
            };
        }
    }
    println!("{}", ans);
}

fn get4(i: usize, j: usize) -> Vec<(usize, usize)> {
    return vec![
        (i    , j    ),
        (i + 1, j    ),
        (i    , j + 1),
        (i + 1, j + 1),
    ];
}
