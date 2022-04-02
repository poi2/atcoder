a, b = gets.chomp.split(" ").map(&:to_i)
xy = Math.sqrt(a * a + b * b)

puts [a / xy, b / xy].join(" ")
