# https://atcoder.jp/contests/abc213/tasks/abc213_b

N = gets.to_i
An = gets.chomp.split(" ").map(&:to_i)

m1, m2 = [0, 0], [0, 0]
An.each_with_index { |v, i|
  x = [v, i + 1]
  if (x <=> m1) == 1
    m2 = m1
    m1 = x
  elsif (x <=> m2) == 1
    m2 = x
  end
}
puts m2[1]
