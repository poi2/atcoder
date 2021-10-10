# https://atcoder.jp/contests/abc095/tasks/arc096_a

a, b, c, x, y = gets.chomp.split(" ").map(&:to_i)

ans = if (a + b) > c * 2
  n = x < y ? x : y
  x -= n
  y -= n
  ans = [
    n * c * 2,
    a > c * 2 ? x * c * 2 : x * a,
    b > c * 2 ? y * c * 2 : y * b
  ].sum
else
  a * x + b * y
end
puts ans
