# https://atcoder.jp/contests/abc221/tasks/abc221_a

n = gets.chomp.to_i
map = Hash.new(0)

n.times.each {
  a, b = gets.chomp.split(" ").map(&:to_i)
  map[a]   += 1
  map[a+b] -= 1
}
# p map
ans = Array.new(n+1) { 0 }

size = 0
before_day = 0
map.to_a.sort.each { |day, diff|
  diff_day = day - before_day

  ans[size] += diff_day

  before_day = day
  size += diff
}
puts ans[1..n].join(" ")
