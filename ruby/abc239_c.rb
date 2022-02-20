x1, y1, x2, y2 = gets.chomp.split(" ").map(&:to_i)

def get_eight(x, y)
  [
    [x + 1, y + 2],
    [x + 2, y + 1],

    [x - 1, y + 2],
    [x - 2, y + 1],

    [x - 1, y - 2],
    [x - 2, y - 1],

    [x + 1, y - 2],
    [x + 2, y - 1],
  ]
end

if (get_eight(x1, y1) & get_eight(x2, y2)).size > 0
  puts "Yes"
else
  puts "No"
end

