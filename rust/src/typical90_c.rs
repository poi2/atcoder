#[warn(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::collections::VecDeque;

fn bfs(n: usize, start: usize, list: &Vec<Vec<usize>>) -> (usize, i64) {
    let mut dist = vec![-1; n];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    dist[start] = 0;

    while let Some(cur_pos) = queue.pop_front() {
        for &next_pos in list[cur_pos].iter() {
            if dist[next_pos] < 0 {
                queue.push_back(next_pos);
                dist[next_pos] = dist[cur_pos] + 1;
            }
        }
    }
    let max_idx = dist.iter().position_max().unwrap();
    (max_idx, dist[max_idx])
}

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    };

    let mut list: Vec<Vec<usize>> = vec![vec![]; n];
    for (_a, _b) in abn.iter() {
        let a = _a - 1;
        let b = _b - 1;
        list[a].push(b);
        list[b].push(a);
    }

    let (p, _) = bfs(n, 0, &list);
    let (_, ans) = bfs(n, p, &list);

    println!("{}", ans + 1);
}
