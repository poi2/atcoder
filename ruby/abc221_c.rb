# https://atcoder.jp/contests/abc221/tasks/abc221_a

n = gets.chomp.chars.map(&:to_i)
list = n.sort.reverse

r = 0
l = 0

list.each { |i|
  if r < l
    r = r * 10 + i
  else
    l = l * 10 + i
  end
}
p r * l
