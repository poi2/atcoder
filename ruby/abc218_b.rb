# # https://atcoder.jp/contests/abc218/tasks/abc218_b

pi = gets.chomp.split(" ").map(&:to_i)
str = [*'a'..'z']

puts pi.map { |i| str[i-1] }.join
