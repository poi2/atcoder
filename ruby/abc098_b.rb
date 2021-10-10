# https://atcoder.jp/contests/abc098/tasks/abc098_b

n = gets.chomp.to_i
list = gets.chomp.chars

puts (0..n-1).map { |i|
  (list[0..i].uniq & list[i+1..-1].uniq).count
}.max

