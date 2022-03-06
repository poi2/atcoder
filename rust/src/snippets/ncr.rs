pub struct NCR {
    memo: Vec<Vec<Option<u128>>>,
}
impl NCR {
    pub fn new(max_n: usize, max_r: usize,) -> Self {
        return Self { memo: vec![vec![None; max_r + 1]; max_n + 1] };
    }

    pub fn call(&mut self, n: usize, r: usize) -> u128 {
        return self.sub_call(n, r);
    }

    pub fn sub_call(&mut self, n: usize, r: usize) -> u128 {
        if r == 0 || n == r { return 1; }
        if let Some(ans) = self.memo[n][r] { return ans; }

        let ans = self.sub_call(n - 1, r - 1) + self.sub_call(n - 1, r);
        self.memo[n][r] = Some(ans);
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ncr_works() {
        let mut ncr = NCR::new(4, 4);
        assert_eq!(ncr.call(4, 4), 1);
        assert_eq!(ncr.call(4, 3), 4);
        assert_eq!(ncr.call(4, 2), 6);
        assert_eq!(ncr.call(4, 1), 4);
        assert_eq!(ncr.call(4, 0), 1);
    }
}