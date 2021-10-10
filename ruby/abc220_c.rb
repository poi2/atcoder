# https://atcoder.jp/contests/abc220/tasks/abc220_c

n = gets.chomp.to_i
a = gets.chomp.split(" ").map(&:to_i)
x = gets.chomp.to_i

suma = a.sum

times = x / suma
remain = x % suma

count = times * n

i = 0
while remain > -1
  remain -= a[i]
  i += 1
  count += 1
end

puts count
