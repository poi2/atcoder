# https://atcoder.jp/contests/abc213/tasks/abc213_a

A, B = gets.chomp.split(" ").map(&:to_i)
an = A.to_s(2).chars.reverse
bn = B.to_s(2).chars.reverse
str = (0..[an.size, bn.size].max - 1).map { |i|
  case [an[i] || '0', bn[i] || '0']
  when ['1', '0'], ['0', '1']
    '1'
  else
    '0'
  end
}.join
puts str.reverse.to_i(2)
