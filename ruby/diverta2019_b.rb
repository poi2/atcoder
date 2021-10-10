# https://atcoder.jp/contests/diverta2019/tasks/diverta2019_b

R, G, B, N = gets.chomp.split(" ").map(&:to_i)

count = 0
(0..N/R).each { |r|
  (0..N/G).each { |g|
    remain = N - (R*r + G*g)
    break if remain < 0
    count += 1 if remain % B == 0
  }
}
puts count
