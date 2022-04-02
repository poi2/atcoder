use proconio::input;

fn main() {
    input! {
        n: usize, mut k: usize, x: usize,
        mut an: [usize; n],
    }
    an.sort();
    an.reverse();

    let mut bn = vec![];
    let mut i = 0;
    let mut cost = an.iter().sum::<usize>();
    while k > 0 && i < n {
        if an[i] > x {
            let mut used = an[i] / x;
            if used > k {
                used = k
            }
            let sub = x * used;
            cost -= sub;
            k -= used;
            let rem = an[i] - sub;
            if rem > 0 {
                bn.push(rem);
            }
        } else {
            bn.push(an[i]);
        }
        i += 1;
    }
    bn.sort();
    bn.reverse();
    // println!("{:?}, {}, {}", bn, k, cost);
    for i in 0..(k.min(bn.len())) {
        cost -= bn[i];
    }
    println!("{}", cost);
}
