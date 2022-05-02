use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        an: [usize; n],
        bm: [usize; m],
    }
    let sum_a = get_cum_sum_list(an, n + 1);
    let sum_b = get_cum_sum_list(bm, m + 1);

    let mut ans = 0;
    let mut idx_b = m as isize;
    for idx_a in 0..n + 1 {
        while 0 <= idx_b && k < sum_a[idx_a] + sum_b[idx_b as usize] {
            idx_b -= 1;
        }
        if 0 <= idx_b {
            ans = ans.max(idx_a + idx_b as usize);
        }
    }
    println!("{}", ans);
}

fn get_cum_sum_list(list: Vec<usize>, len: usize) -> Vec<usize> {
    let mut cum_sum_list = vec![0; len];
    for i in 0..len - 1 {
        cum_sum_list[i + 1] = cum_sum_list[i] + list[i];
    }
    cum_sum_list
}
