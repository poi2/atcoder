_n = gets.chomp.to_i
an = gets.chomp.split(" ").map(&:to_i)
puts an.uniq.size
