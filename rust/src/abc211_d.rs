#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        abm: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in abm {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let counter = bfs(graph, n, 10_usize.pow(9) + 7);
    println!("{}", counter[n - 1]);
}

// NOTE 幅優先探索
fn bfs(graph: Vec<Vec<usize>>, n: usize, mod_num: usize) -> Vec<usize> {
    let max = n + 1;
    let goal = n - 1;
    let mut queue = VecDeque::new();
    let mut depth = vec![max; n];
    let mut counter = vec![0; n];

    queue.push_back(0);
    depth[0] = 0;
    counter[0] = 1;

    while let Some(i) = queue.pop_front() {
        // NOTE ゴールよりも深さが同じか遠い場合、ゴールへの最短経路にはならないため探索を打ち切る（枝刈り）。
        // queue に入れる際、depth[i] は最短経路で確定されている（BFS なので明らか）ため、イコールの場合も枝刈りしてよい。
        if depth[goal] <= depth[i] { continue; }
        for &j in &graph[i] {
            if depth[j] == max {
                // NOTE 初めて訪れる場合
                depth[j] = depth[i] + 1;
                counter[j] = counter[i];
                queue.push_back(j);
            } else if depth[j] == depth[i] + 1 {
                // NOTE 再来 && 次のノードへの最短経路が保証される場合
                counter[j] += counter[i];
                counter[j] %= mod_num;
            }
        }
    }

    return counter;
}
