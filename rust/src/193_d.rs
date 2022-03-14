use proconio::input;

fn main() {
    input! {
        k: usize,
        s: String,
        t: String,
    }
    let s_chars = s.chars().collect::<Vec<char>>();
    let t_chars = t.chars().collect::<Vec<char>>();
    let mut s_cnt = vec![0_usize; 10];
    let mut t_cnt = vec![0_usize; 10];
    let mut a_cnt = vec![k; 10];

    for i in 0..4 {
        let sid = s_chars[i].to_digit(10).unwrap() as usize;
        s_cnt[sid] += 1;
        let tid = t_chars[i].to_digit(10).unwrap() as usize;
        t_cnt[tid] += 1;

        a_cnt[sid] -= 1;
        a_cnt[tid] -= 1;
    }

    let mut win = 0;
    for i in 1..10 {
        if a_cnt[i] == 0 { continue; }
        for j in 1..10 {
            if a_cnt[j] == 0 { continue; }
            if score(&s_cnt, i) <= score(&t_cnt, j) { continue; }
            if i == j {
                win += a_cnt[i] * (a_cnt[j] - 1);
            } else {
                win += a_cnt[i] * a_cnt[j];
            }
        }
    }

    let all_conbinations = (9 * k - 8) * (9 * k - 9);
    let ans = (win as f64) / (all_conbinations as f64);
    println!("{}", ans);
}

fn score(cnt_original: &Vec<usize>, add: usize) -> usize {
    let mut ans = 0;
    let mut cnt = cnt_original.clone();
    cnt[add] += 1;
    for i in 1..10 {
        ans += i * 10_usize.pow(cnt[i] as u32);
    }
    return ans;
}
