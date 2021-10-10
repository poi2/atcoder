# https://atcoder.jp/contests/abc160/tasks/abc160_c

k, n = gets.chomp.split(" ").map(&:to_i)
a = gets.chomp.split(" ").map(&:to_i).sort
a << (k + a[0])
max = a.each_cons(2).map { |a, b| b - a }.max
puts [max, k - max].max
