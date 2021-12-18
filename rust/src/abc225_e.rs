use proconio::input;
use std::cmp::Ordering;

// can't solve with float accurarcy: sort by y / x
fn arg_cmp(a: (i64, i64), b: (i64, i64)) -> std::cmp::Ordering {
    (a.1 * b.0).cmp(&(a.0 * b.1))
}
fn left_of(point: &(i64, i64)) -> (i64, i64) {
    (point.0 - 1, point.1)
}
fn bottom_of(point: &(i64, i64)) -> (i64, i64) {
    (point.0, point.1 - 1)
}

fn main() {
    input! {
        n: usize,
        mut xyn: [(i64, i64); n],
    }
    // ORDER BY left of 7 ASC
    xyn.sort_unstable_by(|a, b|
        arg_cmp(left_of(a), left_of(b))
    );

    let mut ans = 0;
    let mut prev = (1, 0);
    for xy in  xyn {
        // botom of xy >= left of prev => take xy
        if arg_cmp(bottom_of(&xy), left_of(&prev)) != Ordering::Less {
            prev = xy;
            ans += 1;
        }
    }

    println!("{}", ans);
}
