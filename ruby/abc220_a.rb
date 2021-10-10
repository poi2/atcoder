# https://atcoder.jp/contests/abc220/tasks/abc220_a

a, b, c = gets.chomp.split(" ").map(&:to_i)

i = 1
while true
  x = c * i
  if a <= x && x <= b
    puts x
    exit
  elsif x > b
    puts -1
    exit
  end
  i += 1
end
