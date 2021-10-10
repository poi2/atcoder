# https://atcoder.jp/contests/abc064/tasks/abc064_c

_ = gets.chomp.to_i
list = gets.chomp.split(" ").map(&:to_i)

def basic_color(point)
  case point
  when 1..399
    1
  when 400..799
    2
  when 800..1199
    3
  when 1200..1599
    4
  when 1600..1999
    5
  when 2000..2399
    6
  when 2400..2799
    7
  when 2800..3199
    8
  end
end

basic_count = list.map { |point| basic_color(point) }.uniq.count { |i| !!i }
special_count = list.count { |point| point >= 3200 }

if basic_count > 0
  puts [basic_count, basic_count + special_count].join(" ")
else
  puts [1, special_count].join(" ")
end
