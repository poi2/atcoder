use proconio::input;

// TODO DP でも解くこともできそう
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut ans = 1 << 30;
    for bit in 0..(1 << (n - 1)) {
        let mut xor = 0;
        let mut oror = an[0];
        for i in 1..n {
            // 区切る場合
            if bit & 1 << (i - 1) > 0 {
               xor ^= oror;
               oror = 0;
            }
            oror |= an[i];
        }
        xor ^= oror;

        ans = ans.min(xor);
    }
    println!("{}", ans);
}
