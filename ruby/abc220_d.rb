# https://atcoder.jp/contests/abc220/tasks/abc220_d

n = gets.chomp.to_i
an = gets.chomp.split(" ").map(&:to_i)

limit = 998244353

map = {}
(0..9).each { |i|
  (0..9).each { |j|
    map[i * 10 + j] = [
      (i + j) % 10,
      (i * j) % 10,
    ]
  }
}


is = Hash.new(0)
is[an.shift] = 1
(n - 1).times {
  j = an.shift
  new_is = Hash.new(0)
  is.each { |i, times|
    x, y = map[i * 10 + j]
    new_is[x] += times
    new_is[y] += times
  }
  is = new_is
}

(0..9).each { |i|
  puts is[i] % limit
}
