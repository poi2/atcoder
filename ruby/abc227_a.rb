n, k, a = gets.chomp.split(" ").map(&:to_i)

a = a - 1
rem = k % n - 1
ans = (a + rem) % n
puts ans + 1
