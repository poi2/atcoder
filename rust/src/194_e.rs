use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        an: [usize; n],
    }
    let max = 15 * 10_usize.pow(5) + 1;
    let dict = an.iter()
        .enumerate()
        .fold(vec![vec![]; max], |mut acc, (i, &a)| {
            acc[a].push(i);
            return acc;
        });

    match (0..max).find(|&i| check(&dict, i, n, m)) {
        Some(ans) => println!("{}", ans),
        _ => panic!("Error"),
    }
}

fn check(dict: &Vec<Vec<usize>>, i: usize, n: usize, m: usize) -> bool {
    if dict[i].is_empty() { return true; }
    if dict[i].len() == 1 && (m <= dict[i][0] || dict[i][0] + m < n) { return true; }
    for ary in dict[i].windows(2) {
        if ary[1] - ary[0] > m { return true; }
    }
    return false;
}
