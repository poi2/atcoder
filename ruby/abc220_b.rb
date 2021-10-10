# https://atcoder.jp/contests/abc220/tasks/abc220_b

k = gets.chomp.to_i
a, b = gets.chomp.split(" ")

sum = 1
sum *= a.chars.reverse.map.with_index { |d, i| d.to_i * (k ** i) }.sum
sum *= b.chars.reverse.map.with_index { |d, i| d.to_i * (k ** i) }.sum

puts sum
