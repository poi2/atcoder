# https://atcoder.jp/contests/abc157/tasks/abc157_c

N, M = gets.chomp.split(" ").map(&:to_i)
SC = M.times.map { gets.chomp.split(" ").map(&:to_i) }

ds = [nil] * N

SC.each { |s, c|
  d = ds[s-1]
  if d == nil || d == c
    ds[s-1] = c
  else
    puts -1
    exit
  end
}

ds = [0] if ds == [nil]

joined = ds.map.with_index { |x, i|
  if x.nil?
    i == 0 ? 1 : 0
  else
    x
  end
}.join

puts joined == joined.to_i.to_s ? joined.to_i : -1
