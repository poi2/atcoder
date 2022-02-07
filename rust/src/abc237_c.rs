#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use petgraph::unionfind::UnionFind;

extern crate regex;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }
    let chars = s.chars().collect_vec();
    let mut left = 0;
    let mut right = chars.len() - 1;
    let a = 'a';

    while left < right && chars[left] == a && chars[right] == a {
        left += 1;
        right -= 1;
    }
    while left < right && chars[right] == a {
        right -= 1;
    }
    while left < right {
        if chars[left] != chars[right] {
            println!("No");
            return;
        }
        left += 1;
        right -= 1;
    }
    println!("Yes");
}

// 文字列操作が面倒で正規表現でがんばったが、あまり上手い手段ではなかった
fn main_regexp() {
    input! {
        s: String,
    }

    let re1 = Regex::new(r"^a+").unwrap();
    let caps1 = re1.captures(&s);
    let re2 = Regex::new(r"a+$").unwrap();
    let caps2 = re2.captures(&s);

    let mut s1 = "".to_string() + &s;
    if let Some(c2) = caps2 {
        let len1 = if let Some(c1) = caps1 { c1[0].len() } else { 0 };
        let len2 = c2[0].len();
        if len2 > len1 {
            s1 = "a".to_string().repeat(len2 - len1) + &s
        }
    }

    let mut rev_chars = s1.chars().collect_vec();
    rev_chars.reverse();
    let s2 = rev_chars.iter().map(|s| s.to_string()).join("");

    println!("{}", if s1 == s2 { "Yes" } else { "No" });
}
