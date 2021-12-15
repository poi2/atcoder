n, m = gets.chomp.split(" ").map(&:to_i)

list = [*1..n]
rules = []
ab = m.times.map {
  gets.chomp.split(" ").map(&:to_i)
}.sort.uniq.each { |a, b|
  if a > b
    ai = list.find_index(a)
    bi = list.find_index(b)
    if ai > bi
      list[ai] = b
      list[bi] = a
    else
      puts -1
      exit
    end
  else
    rules << [a, b]
  end
}

map = {}
rules.each { |a, b|
  ai = if map[a]
    map[a]
  else
    map[a] = list.find_index(a)
  end

  bi = if map[b]
    map[b]
  else
    map[b] = list.find_index(b)
  end

  if ai > bi
    puts -1
    exit
  end
}
puts list.join(" ")
