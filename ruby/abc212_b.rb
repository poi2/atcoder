# https://atcoder.jp/contests/abc212/tasks/abc212_b

a, b, c, d = gets.chomp.split("").map(&:to_i)

ans =
  if a == b && a == c && a == d
    'Weak'
  elsif [b, c, d] == [(a + 1)%10, (a + 2)%10, (a + 3)%10]
    'Weak'
  else
    'Strong'
  end
puts ans
