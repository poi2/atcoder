# https://atcoder.jp/contests/abc215/tasks/abc216_a

x,y = gets.chomp.split(".").map(&:to_i)

ans = if 0 <= y && y <= 2
  "#{x}-"
elsif 3 <= y && y <= 6
  "#{x}"
else
  "#{x}+"
end
puts ans
