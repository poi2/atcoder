# https://atcoder.jp/contests/abc212/tasks/abc212_c

n, m = gets.chomp.split(" ").map(&:to_i)
a = gets.chomp.split(" ").map(&:to_i).sort
b = gets.chomp.split(" ").map(&:to_i).sort

i, j = 0 , 0
min = 10000000000
while i < n && j < m
  if a[i] == b[j]
    min = 0
    break
  elsif a[i] > b[j]
    diff = a[i] - b[j]
    min = min < diff ? min : diff
    j += 1
  elsif a[i] < b[j]
    diff = b[j] - a[i]
    min = min < diff ? min : diff
    i += 1
  else
    raise
  end
end
puts min
