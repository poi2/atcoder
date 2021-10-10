# https://atcoder.jp/contests/abc122/tasks/abc122_b

s = gets.chomp.chars
acgt = ['A', 'C', 'G', 'T']

ans = 0
counter = 0
s.each { |c|
  if acgt.include? c
    counter += 1
  else
    ans = counter if ans < counter
    counter = 0
  end
}
puts ans < counter ? counter : ans
