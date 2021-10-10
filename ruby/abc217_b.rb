# https://atcoder.jp/contests/abc217/tasks/abc217_b

a = gets.chomp
b = gets.chomp
c = gets.chomp
list = %w(ABC ARC AGC AHC)
puts list - [a, b, c]
