use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize, upper: isize,
        list: [(usize, usize, isize); n],
    }

    let mut pileup = BTreeMap::new();
    for (mut a, b, c) in list {
        a -= 1;
        if let Some(x) = pileup.get_mut(&a) {
            *x += c;
        } else {
            pileup.insert(a, c);
        }
        if let Some(x) = pileup.get_mut(&b) {
            *x -= c;
        } else {
            pileup.insert(b, -1 * c);
        }
    }

    let mut pre_idx = 0;
    let mut pre_cost = if let Some(x) = pileup.get(&0) { *x } else { 0 };
    let mut ans = 0;
    for (&i, &h) in pileup.iter() {
        if i == 0 { continue; }
        ans += (i - pre_idx) * pre_cost.min(upper) as usize;
        pre_idx = i;
        pre_cost = pre_cost + h;
    }
    println!("{}", ans);
}
