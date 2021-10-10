# https://atcoder.jp/contests/abc221/tasks/abc221_a

a, b = gets.chomp.split(" ").map(&:to_i)

puts 32 ** (a - b)
