# https://atcoder.jp/contests/abc063/tasks/arc075_a

n = gets.to_i

min = 101
total = 0
n.times.map {
  i = gets.to_i
  total += i
  min = min > i && i % 10 != 0 ? i : min
}

puts if total % 10 != 0
    total
  elsif min != 101
    total - min
  else
    0
  end
