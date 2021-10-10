# https://atcoder.jp/contests/abc086/tasks/arc089_a

N = gets.chomp.to_i

t1, x1, y1 = 0, 0, 0
n = 0
while n < N
  t2, x2, y2 = gets.chomp.split(" ").map(&:to_i)
  period = t2 - t1
  dist = (x2 - x1).abs + (y2 - y1).abs
  if dist <= period && ((period.odd? && dist.odd?) || (period.even? && dist.even?))
    t1, x1, y1 = t2, x2, y2
    n += 1
  else
    puts "No"
    exit
  end
end

puts "Yes"

