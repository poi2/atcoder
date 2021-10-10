# https://atcoder.jp/contests/abc218/tasks/abc218_d
# https://atcoder.jp/contests/abc218/submissions/25780847 # スマート

def to_key(point)
  "#{point[0]},#{point[1]}"
end

n = gets.chomp.to_i

memo = {}
points = n.times.map {
  point = gets.chomp.split(" ").map(&:to_i)
  memo[to_key(point)] = true
  point
}

count = 0
points.combination(2).each { |p1, p2|
  next if p1[0] == p2[0] || p1[1] == p2[1]
  p3 = [p1[0], p2[1]]
  p4 = [p2[0], p1[1]]

  count += 1 if memo[to_key(p3)] && memo[to_key(p4)]
}
puts count / 2
