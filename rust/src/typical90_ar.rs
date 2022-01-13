#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut an: [usize; n],
    }
    let mut wind: usize = 0;
    let mut ans_list =vec![];

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        }
        match t {
            1 => {
                let ex = (n + x - 1 - wind) % n;
                let ey = (n + y - 1 - wind) % n;
                let tmp = an[ex];
                an[ex] = an[ey];
                an[ey] = tmp;
            },
            2 => {
                wind = (wind + 1) % n;
            },
            3 => {
                let idx = (n + x - 1 - wind) % n;
                ans_list.push(an[idx])
            },
            _ => {},
        }
    }
    for ans in ans_list {
        println!("{}", ans)
    }
}
