# https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c

n, m = gets.chomp.split(" ").map(&:to_i)
aij = n.times.map { gets.chomp.split(" ").map(&:to_i) }

ans = 0
(0..(m-2)).each { |t1|
  ((t1+1)..(m-1)).each { |t2|
    total = (0...n).sum { |i|
      a1 = aij[i][t1]
      a2 = aij[i][t2]
      a1 < a2 ? a2 : a1
    }
    ans = total if ans < total
  }
}
puts ans
