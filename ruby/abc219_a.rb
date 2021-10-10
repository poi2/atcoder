# https://atcoder.jp/contests/abc219/tasks/abc219_a

x = gets.chomp.to_i

y = if x >= 0 && x < 40
    40
  elsif x >= 40 && x < 70
    70
  elsif x >= 70 && x < 90
    90
  elsif x >= 90
    puts "expert"
    exit
  end
puts y - x
