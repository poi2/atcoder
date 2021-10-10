# https://atcoder.jp/contests/abc219/tasks/abc219_d

n = gets.chomp.to_i
x, y = gets.chomp.split(" ").map(&:to_i)
list_ab = n.times.map { gets.chomp.split(" ").map(&:to_i) }

total_x, total_y = 0, 0
list_ab.each { |a, b|
  total_x += a
  total_y += b
}
if total_x < x || total_y < y
  -1
  exit
end

i = 1
min_size = 500
while i < 2 ** n - 1
  bit = i.to_s(2).chars.map(&:to_i)
  size = bit.size
  cur_size = size - bit.count(&:zero?)
  sum_x, sum_y = 0, 0
  bit.each { |idx|
    next if idx == 0
    dx, dy = list_ab[size - idx]
    sum_x += dx
    sum_y += dy
  }
  if x < sum_x && y < sum_y && min_size > cur_size
    min_size = cur_size
  end
  i += 1
end

ans = if min_size == 500
  -1
else
  min_size
end
puts ans
