// https://atcoder.jp/contests/abc128/tasks/abc128_c
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut switches = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            s: [i64; k]
        }
        switches.push(s);
    }
    input! {
        p: [usize; m]
    }

    // スイッチが３つあったとして、
    // bit: 0b111 はすべてのスイッチが付いている状態
    // bit: 0b100 は３つめのスイッチがついている状態
    // 1<<n は左シフトを n 回実行の意味
    // 1<<0 == 0b0, 1<<1 == 0b10, 1<<2 == 0b100
    // bit と左シフトの結果の論理積を取って、0 でなければスイッチが付いている状態
    // スイッチが付いている状態の 2 で割ったあまりが pi と一致すれば、電球 i は付いている状態

    let mut ans: u32 = 0;
    for bit in 0..(1 << n) {
        let mut ac = true;
        for mi in 0..m {
            if ac {
                let mut count = 0;
                for &si in &switches[mi] {
                    if bit & (1 << (si - 1)) != 0 {
                        count += 1;
                    }
                }
                if count % 2 != p[mi] {
                    ac = false;
                }
            }
        }
        if ac {
            ans += 1;
        }
    }
    println!("{}", ans);
}
