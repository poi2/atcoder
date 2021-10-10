# https://atcoder.jp/contests/abc126/tasks/abc126_c

N, K = gets.chomp.split(" ").map(&:to_i)

min = [K-1, N].min
head = (1..min).sum { |i|
  1/N.to_f * (1/2.to_f ** Math.log2(K/i.to_f).ceil)
}
tail = (N - min) * 1/N.to_f

puts head + tail
# N = 4
# K = 10

# 1: 1/4 * (1/2 ** 4)
# 2: 1/4 * (1/2 ** 3)
# 3: 1/4 * (1/2 ** 2)
# 4: 1/4 * (1/2 ** 2)

# N = 5
# K = 8

# 1: 1/5 * (1/2 ** 3)
# 2: 1/5 * (1/2 ** 2)
# 3: 1/5 * (1/2 ** 2)
# 4: 1/5 * (1/2 ** 1)
# 5: 1/5 * (1/2 ** 1)

# N = 10000
# K = 5
# 1: 1/N * (1/2 ** Math.log2(K/1).ceil)
# i: 1/N * (1/2 ** Math.log2(K/i).ceil)
# 5: 1/N * (1/2 ** Math.log2(K/5).ceil) == 1/N
