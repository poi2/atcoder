# https://atcoder.jp/contests/abc215/tasks/abc215_c
s, k = gets.chomp.split(" ")

puts s.chars.permutation.sort.uniq[k.to_i - 1].join
