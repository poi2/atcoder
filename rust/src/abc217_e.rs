#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    let mut sorted = BinaryHeap::new();

    for _ in 0..q {
        input! { c: usize }
        match c {
            1 => {
                input! { x: usize }
                queue.push_back(x);
            },
            2 => {
                if sorted.is_empty() {
                    if let Some(x) = queue.pop_front() {
                        println!("{}", x);
                    }
                } else {
                    if let Some(Reverse(x)) = sorted.pop() {
                        println!("{}", x);
                    }
                }
            },
            3 => {
                while let Some(x) = queue.pop_front() {
                    sorted.push(Reverse(x));
                }
            },
            _ => (),
        }
    }
}
