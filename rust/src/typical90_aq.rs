#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize, w: usize,
        rs: usize, cs: usize,
        rt: usize, ct: usize,
    }
    let mut map: Vec<Vec<bool>> = vec![vec![false; w + 2]; h + 2];
    for i in 0..h {
        input! {
            str: String,
        }
        map[i + 1] = str
            .split("")
            .map(|s| if s == "." { true } else { false } )
            .collect();
    }

    let mut dist = vec![vec![vec![std::usize::MAX; 4]; w + 2]; h + 2];
    let mut que = VecDeque::new();
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for i in 0..4 {
        dist[rs][cs][i] = 0;
        que.push_back((rs, cs, i));
    }

    while let Some((x, y, i)) = que.pop_front() {
        for j in 0..4 {
            let inx = x as i32 + dirs[j].0;
            let iny = y as i32 + dirs[j].1;
            let nx = if inx < 0 { (inx * -1) as usize } else { inx as usize };
            let ny = if iny < 0 { (iny * -1) as usize } else { iny as usize };

            if !map[nx][ny] { continue; }

            let add_cost = if i == j { 0 } else { 1 };
            if dist[nx][ny][j] > dist[x][y][i] + add_cost {
                dist[nx][ny][j] = dist[x][y][i] + add_cost;
                que.push_back((nx, ny, j));
            }
        }
    }
    let &ans = dist[rt][ct].iter().min().unwrap();
    println!("{}", ans);
}
