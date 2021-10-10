# https://atcoder.jp/contests/abc068/tasks/abc068_b

require 'prime'

n = gets.chomp.to_i
puts (1..n).map { |i| t = i.prime_division.find{|n, _| n == 2}&.last || 0; [t, i] }.max.last
