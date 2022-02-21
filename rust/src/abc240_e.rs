#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashMap;

const MIN_INIT: usize = 200_001;
const MAX_INIT: usize = 1;

fn main() {
    input! {
        n: usize,
        uvn: [(usize, usize); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uvn {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let leaf = dfs_leaf_search(&graph, HashMap::new(), 0, None);
    // println!("{:?}", leaf);

    let ans = dfs(0, None, vec![(MIN_INIT, MAX_INIT); n], &graph, &leaf);
    // println!("{:?}", ans);
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn dfs_leaf_search(graph: &Vec<Vec<usize>>, mut leaf: HashMap<usize, usize>, id: usize, parent_id: Option<usize>) -> HashMap<usize, usize> {
    if graph[id].len() == 1 && parent_id != None {
        leaf.insert(id, leaf.len() + 1);
        return leaf;
    }

    for &child_id in graph[id].iter() {
        if parent_id != None && child_id == parent_id.unwrap() { continue; }
        leaf = dfs_leaf_search(graph, leaf, child_id, Some(id));
    }

    return leaf;
}

fn dfs(id: usize, parent_id: Option<usize>, mut ans: Vec<(usize, usize)>, graph: &Vec<Vec<usize>>, leaf: &HashMap<usize, usize>) -> Vec<(usize, usize)> {
    if let Some(&leaf_id) = leaf.get(&id) {
        ans[id] = (leaf_id, leaf_id);
        return ans;
    }

    let mut min = MIN_INIT;
    let mut max = MAX_INIT;
    for &child_id in graph[id].iter() {
        if parent_id != None && child_id == parent_id.unwrap() { continue; }
        ans = dfs(child_id, Some(id), ans, graph, leaf);
        min = min.min(ans[child_id].0);
        max = max.max(ans[child_id].1);
    }
    ans[id] = (min, max);

    return ans;
}
