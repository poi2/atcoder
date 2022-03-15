use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, m: usize, x: usize, y: usize,
        list: [(usize, usize, usize, usize); m],
    }

    let graph = list
        .iter()
        .fold(vec![vec![]; n], |mut acc, &(mut a, mut b, t, k)| {
            a -= 1;
            b -= 1;
            acc[a].push((b, t, k));
            acc[b].push((a, t, k));
            return acc;
        });

    match dijkstra(x - 1, y - 1, n, graph) {
        None => println!("{}", -1),
        Some(n) => println!("{}", n),
    }

}

fn dijkstra(s: usize, e: usize, n: usize, graph: Vec<Vec<(usize, usize, usize)>>) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), s));
    let max = std::usize::MAX;
    let mut used = vec![max; n];
    used[s] = 0;

    while let Some((Reverse(dist), cur)) = queue.pop() {
        // queue に (dist, cur) を入れた後、別の要素でより近いパスが見つかったケースをスキップさせる
        if used[cur] < dist { continue; }
        for &(nxt, t, k) in &graph[cur] {
            let nxt_dist = dist + ((k - dist % k) % k) + t;
            if nxt_dist < used[nxt] {
                used[nxt] = nxt_dist;
                queue.push((Reverse(nxt_dist), nxt));
            }
        }
    }

    return match used[e] {
        std::usize::MAX => None,
        n => Some(n),
    };
}
