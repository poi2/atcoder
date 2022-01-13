n = gets.chomp.to_i
xyn = n.times.map {
  gets.chomp.split(" ").map(&:to_i)
}

list = (0...n).flat_map { |i|
  (0...n).flat_map { |j|
    x1, y1 = xyn[i]
    x2, y2 = xyn[j]
    Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)
  }
}
puts list.sort.last
