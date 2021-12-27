use itertools::Itertools;
use proconio::input;

fn get_index(target: &str, chars: &Vec<&str>) -> i32 {
    let (idx, _) = chars.iter().find_position(|&&c| c == target).unwrap();
    (idx as i32) % 26
}

fn main() {
    input! {
        s: String,
        t: String,
    };
    let slist: Vec<&str> = s.split("").collect();
    let tlist: Vec<&str> = t.split("").collect();

    let chars = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let i = 1;
    let diff = (get_index(slist[i], &chars) - get_index(tlist[i], &chars) + 26) % 26;

    let mut ans = "Yes";
    for i in 0..slist.len() {
        if slist[i] != "" {
            let sidx = get_index(slist[i], &chars);
            let tidx = get_index(tlist[i], &chars);
            if ((sidx - tidx + 26) % 26) != diff {
                ans = "No";
            }
        }
    }
    println!("{}", ans);
}
