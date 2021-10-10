# https://atcoder.jp/contests/arc073/tasks/arc073_a

N, T = gets.chomp.split(" ").map(&:to_i)
ts = gets.chomp.split(" ").map(&:to_i)

total = T
ts.each_cons(2) { |x, y|
  diff = y - x
  if diff <= T
    total += diff
  else
    total += T
  end
}
puts total
