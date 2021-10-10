# https://atcoder.jp/contests/abc088/tasks/abc088_c

list = 3.times.map { gets.chomp.split(" ").map(&:to_i) }
min = list[0].min
a, b, c = list[0].map{ |i| i - min }
puts list[1..2].all? { |x, y, z| x - a == y - b && x - a == z - c } ? 'Yes' : 'No'
