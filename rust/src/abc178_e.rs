use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(isize, isize); n],
    }
    let mut z = vec![];
    let mut w = vec![];
    for i in 0..n {
        let (x, y) = xyn[i];
        z.push(x + y);
        w.push(x - y);
    }
    z.sort();
    w.sort();
    let last = n - 1;
    let ans = (z[last] - z[0]).max(w[last] - w[0]);
    println!("{}", ans);
}
