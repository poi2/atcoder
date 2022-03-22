use proconio::input;
use proconio::source::line::LineSource;
use std::io;
use std::io::BufReader;
use std::collections::HashSet;
use std::iter::FromIterator;

// NOTE インタラクティブに入出力を介して対話するプログラム
fn main() {
    // この２行のマクロを使うことで、標準出力を出力後、出力の削除 flush を行う
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {
        n: usize,
    }
    let max = 2 * n + 1;
    let mut set: HashSet<usize> = HashSet::from_iter(1..=max);
    while let Some(&i) = set.iter().next() {
        println!("{:?}", i);

        input! { j: usize, }
        if j == 0 { break; }

        set.remove(&i);
        set.remove(&j);
    }
}
