s, t, x = gets.chomp.split(" ").map(&:to_i)

if s < t
  if s <= x && x < t
    puts 'Yes'
  else
    puts 'No'
  end
elsif s == t
  puts 'No'
else
  if s <= x || x < t
    puts 'Yes'
  else
    puts 'No'
  end
end
