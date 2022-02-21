#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let mut anss = vec![];
    for _ in 0..t {
        input! {
            n: usize, _m: usize,
            xyn: [(isize, usize); n],
        }

        let mut bn: Vec<isize> = vec![0; n + 1];
        for i in 0..n {
            let (x, y) = xyn[i];
            bn[i + 1] = bn[i] + x * (y as isize);
        }

        let mut a0 = 0_isize;
        let mut ans = xyn[0].0;
        for i in 0..n {
            let (x, y_usize) = xyn[i];
            let y = y_usize as isize;
            let a1 = dist(a0, bn[i], x, y);
            ans = ans.max(a1);
            if bn[i] > 0 && bn[i + 1] < 0 {
                let ydash = -1 * bn[i] / x;
                let mid = dist(a0, bn[i], x, ydash);
                ans = ans.max(mid);
            }
            a0 = a1;
        }
        anss.push(ans);
    }

    anss.iter().for_each(|ans| println!("{}", ans));
}

fn dist(dist: isize, speed: isize, acceleration: isize, times: isize) -> isize {
    return dist + speed * times + acceleration * times * (times + 1) / 2;
}
