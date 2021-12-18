n = gets.chomp.to_i

map = Hash.new {0}
n.times {
  s = gets.chomp
  map[s] = map[s] + 1
}
puts map.to_a.sort_by{|k, v| v}.last.first
