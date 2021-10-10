# https://atcoder.jp/contests/abc074/tasks/abc074_b

n = gets.chomp.to_i
k = gets.chomp.to_i
xs = gets.chomp.split(" ").map(&:to_i)

puts xs.sum { |x| [x, (k - x).abs].min } * 2
