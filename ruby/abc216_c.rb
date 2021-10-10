# https://atcoder.jp/contests/abc216/tasks/abc216_c

n = gets.chomp.to_i

ans = []
while n != 0
  if n.even?
    ans << "B"
    n /= 2
  else
    ans << "A"
    n -= 1
  end
end
puts ans.reverse.join
