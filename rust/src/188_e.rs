use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        an: [usize; n],
        xym: [(usize, usize); m],
    }
    let mut roots = BTreeSet::new();
    for i in 0..n { roots.insert(i); }
    let mut graph = vec![vec![]; n];
    for (mut x, mut y) in xym {
        x -= 1;
        y -= 1;
        graph[x].push(y);
        roots.remove(&y);
    }
    let max = 10_usize.pow(9_u32) + 1;
    let mut dp = vec![max; n];

    let mut queue = roots.iter().map(|&i| i).collect::<Vec<usize>>();
    while let Some(v0) = queue.pop() {
        for &v1 in &graph[v0] {
            let v1_min = dp[v0].min(an[v0]);
            if v1_min < dp[v1] {
                dp[v1] = v1_min;
                queue.push(v1);
            }
        }
    }

    let mut ans = -1 * max as isize;
    for i in 0..n {
        if dp[i] == max { continue; }
        ans = ans.max(an[i] as isize - dp[i] as isize);
    }
    println!("{}", ans);
}
