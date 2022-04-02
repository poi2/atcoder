p1 = gets.chomp.split(" ").map(&:to_i)
p2 = gets.chomp.split(" ").map(&:to_i)
p3 = gets.chomp.split(" ").map(&:to_i)

x = [p1[0], p2[0], p3[0]].group_by { |n| n }.find{|k, v| v.count == 1}.first
y = [p1[1], p2[1], p3[1]].group_by { |n| n }.find{|k, v| v.count == 1}.first
puts [x, y].join(" ")
