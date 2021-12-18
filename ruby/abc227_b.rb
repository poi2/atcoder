n = gets.chomp.to_i
sn = gets.chomp.split(" ").map(&:to_i)

list = []
(1..143).each { |a|
  (a..143).each { |b|
    res = 4 * a * b + 3 * a + 3 * b
    list << res if res <= 1000
  }
}
list.sort.uniq
puts (sn - list).count

