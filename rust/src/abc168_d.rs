use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        abm: [(usize, usize); m],
    }

    let mut path = vec![vec![]; n];
    for (mut a, mut b) in abm {
        a -= 1;
        b -= 1;
        path[a].push(b);
        path[b].push(a);
    }

    let mut used = vec![false; n];
    const LIMIT: usize = 1010101;
    let mut signs = vec![LIMIT; n];
    let mut nxt = std::collections::VecDeque::new();

    nxt.push_front(0);
    used[0] = true;
    while let Some(i) = nxt.pop_front() {
        for &j in path[i].iter() {
            if !used[j] {
                used[j] = true;
                signs[j] = i;
                nxt.push_back(j);
            }
        }
    }

    if used.iter().all(|&bool| bool) {
        println!("Yes");
        for i in 1..n {
            println!("{}", signs[i] + 1);
        }
    } else {
        println!("No");
    }
}
