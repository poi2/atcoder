# https://atcoder.jp/contests/abc219/tasks/abc219_b

s = 3.times.map { gets.chomp }
t = gets.chomp.chars
puts t.map { |str| s[str.to_i-1] }.join("")
