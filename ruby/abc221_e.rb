# https://atcoder.jp/contests/abc221/tasks/abc221_e

n = gets.chomp.to_i
list = gets.chomp.split(" ").map(&:to_i)

(0..n-2).each { |i|
  dp = Hash.new
  dp[list[i]] = [1, 1]
  (i+1..n-1).each { |j|
    cur = list[j]
    keys = dp.keys.select { |v| v <= cur }
    p [dp, cur, keys, ]
    dp[cur] += keys.sum{|k| dp[k]}
  }
  # p dp
}



max, size, count