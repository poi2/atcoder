n, q = gets.chomp.split(" ").map(&:to_i)
an = gets.chomp.split(" ").map(&:to_i).sort

ans = []
q.times {
  x = gets.chomp.to_i
  ans << (n - (an.bsearch_index { _1 >= x} || n))
}
puts ans
