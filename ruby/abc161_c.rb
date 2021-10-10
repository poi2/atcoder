# https://atcoder.jp/contests/abc161/tasks/abc161_c

n, k = gets.chomp.split(" ").map(&:to_i)
rem = n % k
puts [rem, k - rem].min
