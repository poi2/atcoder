use proconio::input;
use proconio::marker::Usize1;

// 深さ優先探索 Depth-First Search
fn dfs(i: usize, used: &mut Vec<bool>, edges: &Vec<Vec<usize>>) {
    used[i] = true;
    for &e in &edges[i] {
        if !used[e] {
            dfs(e, used, edges);
        }
    }
}

// 幅優先探索 Breadth-First Search
// fn bfs(n: usize, used: &mut Vec<bool>, edges: &Vec<Vec<usize>>) {
//     let mut que = vec![n-1];
//     while let Some(i) = que.pop() {
//         used[i] = true;
//         for &to in &edges[i] {
//             if used[to] {
//                 continue;
//             }
//             que.push(to);
//         }
//     }
// }

fn main() {
    input! {
        n: usize,
    }
    let mut times = vec![0; n];
    let mut edges = vec![vec![]; n];

    for i in 0..n {
        input! {
            t: usize,
            k: usize,
            ak: [Usize1; k],
        }
        times[i] = t;
        edges[i] = ak;
    }

    let mut used = vec![false; n];

    dfs(n - 1, &mut used, &edges);

    let mut ans = 0;
    for i in 0..n {
        if used[i] {
            ans += times[i];
        }
    }
    println!("{:?}", ans);
}
