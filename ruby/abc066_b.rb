# https://atcoder.jp/contests/abc066/tasks/abc066_b

# A1
def solve(list, i, max)
  llist = list[0..i-1]
  rlist = list[i..2*i-1]

  llist == rlist ? i * 2 : max
end

list = gets.chomp.split("")
puts (1..(list.size-1)/2).inject(0) { |max, i|
  solve(list, i, max)
}

# A2
list = gets.chomp.chars
len = list.size - 1
i = len.odd? ? len - 1 : len
while i > 1
  break if i.even? && list[0..i/2 - 1] == list[i/2..i-1]
  i -= 2
end
puts i
