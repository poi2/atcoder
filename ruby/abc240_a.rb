a, b = gets.chomp.split(" ").map(&:to_i)

if a + 1 == b || (a == 1 && b == 10)
  puts "Yes"
else
  puts "No"
end
