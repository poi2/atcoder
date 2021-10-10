# https://atcoder.jp/contests/abc073/tasks/abc073_c

N = gets.to_i
An = N.times.map { gets.to_i }
puts An.tally.count { |k, v| v.odd? }
