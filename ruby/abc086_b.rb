# https://atcoder.jp/contests/abc086/tasks/abc086_b

ab = gets.chomp.tr(" ", "").to_i
puts Integer.sqrt(ab) ** 2 == ab ? 'Yes' : 'No'
