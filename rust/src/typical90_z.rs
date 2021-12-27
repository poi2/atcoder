#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn dfs(cur_idx: usize, cur_color: usize, colors: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    colors[cur_idx] = cur_color;
    for &next_idx in &graph[cur_idx] {
        if colors[next_idx] == 9 {
            dfs(next_idx, 1 - cur_color, colors, graph);
        }
    }
}

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    };
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in abn {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    // NOTE red: 0, blue: 1, not used: 9
    let mut colors: Vec<usize> = vec![9; n];

    dfs(0, 0, &mut colors, &graph);

    let red_count = colors.iter().filter(|&&color| color == 0).count();
    let color = if red_count >= (n / 2) { 0 } else { 1 };

    let ans = colors
        .iter()
        .enumerate()
        .filter(|&(_idx, value)| *value == color)
        .map(|(idx, _value)| (idx + 1).to_string())
        .collect::<Vec<String>>()
        .drain(0..n / 2)
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", ans);
}
