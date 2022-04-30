use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let mut ans = int_len(k, 0);
    let mut rem = dup7(0, ans);
    let mut used = std::collections::HashSet::new();
    while rem % k != 0 && !used.contains(&rem) {
        used.insert(rem);

        rem %= k;
        rem = rem * 10 + 7;
        ans += 1;
    }
    if used.contains(&rem) {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

fn int_len(i: usize, len: usize) -> usize {
    if i > 0 {
        int_len(i / 10, len + 1)
    } else {
        len
    }
}
fn dup7(i: usize, digit: usize) -> usize {
    if digit > 0 {
        dup7(i * 10 + 7, digit - 1)
    } else {
        i
    }
}
