# https://atcoder.jp/contests/abc219/tasks/abc219_c

def_alf = ('a'..'z').to_a
map = gets.chomp.chars.each_with_index.each_with_object({}) {|(s, i), acc| acc[s] = def_alf[i] }
n = gets.chomp.to_i

list = n.times.map {
  gets.chomp
}

puts sorted_list = list.each_with_index.map { |str, i|
  [str.chars.map { |s| map[s] }.join, i]
}.sort.map{|_, i| list[i] }.join("\n")



