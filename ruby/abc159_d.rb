# https://atcoder.jp/contests/abc159/tasks/abc159_d

class Integer
  class << self
    # NOTE 階乗
    @@fact_cache = {0 => 1, 1 => 1, 2 => 2, 3 => 6, 4 => 24, 5 => 120}
    def fact(num)
      sign, n = num.negative? ? [-1, num * -1] : [1, num]
      if @@fact_cache[n] == nil
        @@fact_cache[n] = fact(n-1) * n
      end
      @@fact_cache[n] * sign
    end
    def fact_cache; @@fact_cache; end
    def fact_no_cache(n); (2..n).reduce(1, :*); end

    # NOTE 組み合わせ
    @@nrc_cache = {}
    def nCr(n, r)
      return 0 if n <  r
      return 1 if n == r
      if @@nrc_cache[[n, r]] == nil
        @@nrc_cache[[n, r]] = Integer.fact(n) / (Integer.fact(r) * Integer.fact(n - r))
      end
      @@nrc_cache[[n, r]]
    end

    # NOTE 等差数列の和
    def sum_seq(a, n, d)
      n * (2 * a + (n - 1) * d ) / 2
    end
  end
end

N = gets.to_i
An = gets.chomp.split(" ").map(&:to_i)

tally = An.tally
memo = {}

total = tally.sum { |k, v| Integer.sum_seq(1, v - 1, 1) }
(0...N).each { |i|
  ai = An[i]
  if memo[ai]
    memo[ai]
  else
    cnt = tally[ai]
    memo[ai] = total - Integer.sum_seq(1, cnt - 1, 1) + Integer.sum_seq(1, cnt - 2, 1)
  end
  puts memo[ai]
}
