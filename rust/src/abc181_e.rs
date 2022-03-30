use num::Integer;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize, m: usize,
        mut hn: [usize; n],
        mut wm: [usize; m],
    }
    hn.sort();
    let mut front_stack = vec![0; n / 2 + 1];
    for i in 0..n / 2 {
        front_stack[i + 1] = hn[i * 2 + 1] - hn[i * 2] + front_stack[i];
    }
    let mut back_stack = vec![0; n / 2 + 1];
    for i in 0..n / 2 {
        back_stack[i + 1] = hn[n - i * 2 - 1] - hn[n - i * 2 - 2] + back_stack[i];
    }
    back_stack.reverse();

    let mut ans = std::usize::MAX;
    for w in wm {
        let ins_idx = hn.lower_bound(&w);
        let idx2 = ins_idx / 2;
        let sum1 = front_stack[idx2] + back_stack[idx2];
        let sum2 = if ins_idx.is_even() { hn[ins_idx] - w } else { w - hn[ins_idx - 1] };
        ans = ans.min(sum1 + sum2);
    }
    println!("{}", ans);
}
