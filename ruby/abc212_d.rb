# https://atcoder.jp/contests/abc212/tasks/abc212_d

q = gets.chomp.to_i
queries = q.times.map { gets.chomp.split(" ").map(&:to_i) }

bag = []
add = 0
queries.each { |ary|
  case ary[0]
  when 1
    n = ary[1] - add
    idx = bag.bsearch_index { |i| i > n }
    if idx
      bag.insert(idx, n)
    else
      bag.insert(-1, n)
    end
  when 2
    add += ary[1]
  when 3
    puts bag.shift + add
  end
}
