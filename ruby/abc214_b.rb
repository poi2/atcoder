# https://atcoder.jp/contests/abc214/tasks/abc214_b

s, t = gets.chomp.split(" ").map(&:to_i)

count = 0
(0..[100, s].min).each { |a|
  bmax = s - a
  (0..bmax).each { |b|
    cmax = s - a - b
    (0..cmax).each { |c|
      if a + b + c <= s && a * b * c <= t
        count += 1
      end
    }
  }
}
puts count
