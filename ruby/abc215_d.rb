# https://atcoder.jp/contests/abc215/tasks/abc215_d

require 'prime'

_, m = gets.chomp.split(" ").map(&:to_i)
a = gets.chomp.split(" ").map(&:to_i)

dels = a.flat_map { |ai| Prime.prime_division(ai).map(&:first) }.uniq
ans = (0..m).to_a
dels.each { |x|
  base = 1
  while x * base <= m
    ans[x * base] = 0
    base += 1
  end
}
ans.delete_if{ |x| x == 0 }
puts ans.size
puts ans
