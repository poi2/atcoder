# https://atcoder.jp/contests/abc217/tasks/abc217_e

q = gets.chomp.to_i

tmp = []
list = []
ans = []
q.times {
  i, x = gets.chomp.split(" ").map(&:to_i)
  if i == 1
    tmp << x
  elsif i == 2
    if list == []
      ans << list.tmp
    else
      ans << list.shift
    end
  else
    list << tmp
    list.sort!
  end
}
puts ans
