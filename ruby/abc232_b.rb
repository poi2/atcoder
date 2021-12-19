s = gets.chomp.split('')
t = gets.chomp.split('')

list =('a'..'z').to_a

diff = (list.find_index(s[0]) - list.find_index(t[0])) % 26
if (0...s.length).all? { |i| (list.find_index(s[i]) - list.find_index(t[i])) % 26 == diff }
  puts 'Yes'
else
  puts 'No'
end

