# https://atcoder.jp/contests/abc216/tasks/abc216_d

n, m = gets.chomp.split(" ").map(&:to_i)

pair = []
head = {}
a = m.times.map { |i|
  _ = gets
  x = gets.split(" ").map(&:to_i)
  if j = head[x.first]
    pair << [j, i]
  else
    head[x.first] = i
  end
  x
}

count = 0
while pair.size > 0
  pair.pop.each { |idx|
    a[idx].shift
    x = a[idx].first
    if x == nil
      count += 1
    elsif head[x]
      pair << [head[x], idx]
    else
      head[x] = idx
    end
  }
end
puts count == m ? 'Yes' : "No"
