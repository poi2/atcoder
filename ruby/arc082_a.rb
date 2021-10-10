# https://atcoder.jp/contests/abc072/tasks/arc082_a

n = gets.chomp.to_i
list = gets.chomp.split(" ").map(&:to_i).sort
puts list.flat_map { |l| (-1..1).map { |i| l + i  } }.tally.values.max


# https://atcoder.jp/contests/abc072/submissions/22504295
# 別のユーザーさんの回答
# 頭柔らかい

# n = gets.to_i
# a = gets.split.map(&:to_i)
# s = Array.new(a.max+3,0)

# a.each{|i|
#   s[i] += 1
#   s[i+1] += 1
#   s[i+2] += 1
#   }

# p s.max
