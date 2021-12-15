n, k = gets.chomp.split(" ").map(&:to_i)
scores = n.times.map {
  gets.chomp.split(" ").map(&:to_i).sum
}

line = scores.sort[n - k]

ans = []
scores.each { |score|
  if score + 300 >= line
    ans << 'Yes'
  else
    ans << 'No'
  end
}

puts ans
