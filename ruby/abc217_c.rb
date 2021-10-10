# https://atcoder.jp/contests/abc217/tasks/abc217_c

n = gets.chomp.to_i
p_with_i = gets.chomp.split(" ").map(&:to_i).map.with_index{|x, i| [x, i + 1]}
puts p_with_i.sort.map { |x, i| i }.join(" ")
