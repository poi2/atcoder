#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use petgraph::unionfind::UnionFind;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize,
        mut dn: [usize; n],
        abm: [(usize, usize); m],
    }

    // NOTE 0 < M < N - 1 であるから、sum(dn) は必ず 2n - 2 になる
    if dn.iter().sum::<usize>() != 2 * n - 2 {
        println!("-1");
        return;
    }

    let mut tree = UnionFind::new(n + 1);
    for (aa, bb) in abm {
        let a = aa - 1;
        let b = bb - 1;
        if !tree.union(a, b) {
            println!("-1");
            return;
        }
        // NOTE dn の値以上に高速道路が引かれているケースがある
        if dn[a] == 0 || dn[b] == 0 {
            println!("-1");
            return;
        }
        dn[a] -= 1;
        dn[b] -= 1;
    }

    let mut roots = HashSet::new();
    let mut cities = vec![vec![]; n];
    for i in 0..n {
        let root = tree.find(i);
        roots.insert(root);
        if dn[i] > 0 {
            cities[root].push((i, dn[i]));
        }
    }

    let mut v1 = vec![];
    let mut v2 = vec![];
    let mut connections = vec![0; n];
    for &root in roots.iter() {
        let mut connection = 0;
        for &city in &cities[root] {
            connection += city.1;
        }
        match connection {
            0 => (),
            1 => v1.push(root),
            _ => v2.push(root),
        }
        connections[root] = connection;
    }

    let mut sum = 0;
    for &root in &roots {
        for tap in &cities[root] {
            sum += tap.1;
        }
    }
    if sum != (n - m - 1) * 2 {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    while v1.len() > 0 && v2.len() > 0 {
        let root_1 = v1.pop().unwrap();
        let (city_1, _count_1) = cities[root_1].pop().unwrap();
        connections[root_1] -= 1;

        let root_2 = v2.pop().unwrap();
        let (city_2, count_2) = cities[root_2].pop().unwrap();
        connections[root_2] -= 1;
        if connections[root_2] == 1 {
            v1.push(root_2)
        } else {
            v2.push(root_2)
        }
        if count_2 > 1 {
            cities[root_2].push((city_2, count_2 - 1));
        }

        ans.push((city_1, city_2));
    }
    while v1.len() == 2 {
        let root_1 = v1.pop().unwrap();
        let (city_1, _count_1) = cities[root_1].pop().unwrap();
        let root_2 = v1.pop().unwrap();
        let (city_2, _count_2) = cities[root_2].pop().unwrap();
        ans.push((city_1, city_2));
    }

    if v1.len() == 0 && v2.len() == 0 {
        for (c1, c2) in ans { println!("{} {}", c1 + 1, c2 + 1)};
    } else {
        println!("-1");
    }
}
