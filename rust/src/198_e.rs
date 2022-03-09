use proconio::input;

fn main() {
    input! {
        n: usize,
        cn: [usize; n],
        abn: [(usize, usize); n - 1],
    }
    const MAX: usize = 100_001;

    let mut graph = vec![vec![]; n];
    for (a, b) in abn {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut counter = vec![0_usize; MAX];
    let mut ans = vec![false; n];
    dfs(0, MAX, &graph, &cn, &mut counter, &mut ans);
    for i in 0..n {
        if ans[i] {
            println!("{}", i + 1);
        }
    }
}

// NOTE 巡回しないことを前提としている
// もし巡回するなら使用済み頂点 used を持って管理すればよいはず
fn dfs(cur: usize, prev: usize, graph: &Vec<Vec<usize>>, cn: &Vec<usize>, counter: &mut Vec<usize>, ans: &mut Vec<bool>) {
    counter[cn[cur]] += 1;
    if counter[cn[cur]] == 1 { ans[cur] = true; }

    for &next in graph[cur].iter() {
        if next != prev {
            dfs(next, cur, graph, cn, counter, ans);
        }
    }

    counter[cn[cur]] -= 1;
}
