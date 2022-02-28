use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

// NOTE
// BTreeSet には (要素: T, 要素の異なり数: usize) のタプルを入れる
// insert, remove 関数だけ作成している
// 要素の取得は set 関数が返す BTreeSet<(T, usize)> からよしなに行う
pub struct MultiSet<T> {
    set: BTreeSet<(T, usize)>,
    counter: HashMap<T, usize>,
}

impl<T: Clone + Eq + Hash + Ord + Copy> MultiSet<T> {
    pub fn new() -> Self {
        MultiSet { set: BTreeSet::new(), counter: HashMap::new() }
    }

    pub fn insert(&mut self, x: T) {
        let set_id =
            if let Some(id) = self.counter.get_mut(&x) {
                let next_id = *id + 1;
                *id += 1;
                next_id
            } else {
                self.counter.insert(x, 1);
                1
            };
        self.set.insert((x, set_id));
    }

    pub fn remove(&mut self, x: T) {
        if let Some(id) = self.counter.get_mut(&x) {
            self.set.remove(&(x, *id));
            *id -= 1;
        }
    }

    pub fn set(&self) -> &BTreeSet<(T, usize)> {
        &self.set
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_set_works() {
        let mut multi_set = MultiSet::new();
        for i in vec![0, 1, 2, 2, 3, 3, 3] { multi_set.insert(i); }

        assert_eq!(multi_set.set.range(..(1, 0)).count(), 1);
        assert_eq!(multi_set.set.range(..(2, 0)).count(), 2);
        assert_eq!(multi_set.set.range(..(3, 0)).count(), 4);
        assert_eq!(multi_set.set.range(..(4, 0)).count(), 7);

        assert_eq!(multi_set.set.range((2, 0)..(3, 0)).count(), 2);

        multi_set.remove(1);
        multi_set.remove(2);
        assert_eq!(multi_set.set.range(..(2, 0)).count(), 1);
        assert_eq!(multi_set.set.range((2, 0)..(3, 0)).count(), 1);
    }
}
