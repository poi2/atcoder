use proconio::input;

fn main() {
    input! {
        s: [String; 2]
    }
    let ans =
        if (s[0] == ".#" && s[1] == "#.") || (s[0] == "#." && s[1] == ".#") {
            "No"
        } else {
            "Yes"
        };
    println!("{}", ans);
}
