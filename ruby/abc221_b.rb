# https://atcoder.jp/contests/abc221/tasks/abc221_a

s = gets.chomp.chars
t = gets.chomp.chars

len = s.length
diff = (0..len).select { |i| s[i] != t[i]  }

if diff.size == 0
  puts "Yes"
  exit
end

if diff.size == 2
  i, j = diff
  if i + 1 == j && s[i] == t[j] && s[j]  == t[i]
    puts "Yes"
  else
    puts "No"
  end
else
  puts "No"
end

