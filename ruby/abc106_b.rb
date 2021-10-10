# https://atcoder.jp/contests/abc106/tasks/abc106_b

require 'prime'

n = gets.chomp.to_i

i = 1
ans = 0
while i <= n
  ans += 1 if i.prime_division.map { _2 + 1 }.inject(:*) == 8
  i += 2
end
puts ans
