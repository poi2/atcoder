fn main() {
    proconio::input! {
        l: u32,
        q: u32
    }
    let mut vec = Vec::new();

    for i in 0..q-1 {
      proconio::input! {
        c: u32,
        x: u32
      }
      if c == 1 {
        vec.push(x);
      } else {
        vec.sort();
      }
    }
}
