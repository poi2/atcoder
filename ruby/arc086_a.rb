# https://atcoder.jp/contests/arc086/tasks/arc086_a

_, k = gets.chomp.split.map(&:to_i)
an = gets.chomp.split(" ").map(&:to_i)

tally = an.tally
count = tally.count
ans =
  if count <= k
    0
  else
    tally.values.min(count - k).sum
  end
puts ans
