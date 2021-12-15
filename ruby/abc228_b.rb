n, x = gets.chomp.split(" ").map(&:to_i)
an = gets.chomp.split(" ").map{|s| s.to_i - 1}
x -= 1

used = Array.new(n) {false}

i = x
while used[i] == false
  used[i] = true
  next_i = an[i]
  if used[next_i]
    break
  else
    i = next_i
  end
end

puts used.count{ |f| f == true }
