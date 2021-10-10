# https://atcoder.jp/contests/abc212/tasks/abc212_a

a, b = gets.chomp.split.map(&:to_i)

ans = if a > 0 && b == 0
  'Gold'
elsif a == 0 && b > 0
  'Silver'
elsif a > 0 && b > 0
  'Alloy'
end
puts ans
