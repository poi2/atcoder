use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = vec![];
    for bits in 0..1<<n {
        let mut counter = 0;
        let mut valid = true;
        for i in 0..n {
            if (bits >> i & 1) == 1 {
                counter += 1;
            } else {
                counter -= 1;
            }
            if counter < 0 {
                valid = false;
                break;
            }
        }
        let mut str = "".to_string();
        if valid == true && counter == 0 {
            for i in 0..n {
                if (bits >> i & 1) == 1 {
                    str = str + "(";
                } else {
                    str = str + ")";
                }
            }
            ans.push(str);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}