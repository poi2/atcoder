s = gets.chomp.chars

size = s.size
list = []
size.times.map {
  h = s.shift
  s << h
  list << s.join
}
sorted = list.sort

puts sorted.first
puts sorted.last
