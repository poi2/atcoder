# https://atcoder.jp/contests/abc057/tasks/abc057_b

n, m = gets.chomp.split(" ").map(&:to_i)
as = n.times.map { |_| gets.chomp.split(" ").map(&:to_i) }
cs = m.times.map { |_| gets.chomp.split(" ").map(&:to_i) }

puts as.map { |x, y|
  cs.map { |s, t|
    (s - x).abs + (t - y).abs
  }.each_with_index.min { |u, v| u[0] <=> v[0] }[1] + 1
}
