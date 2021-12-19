n, m = gets.chomp.split(" ").map(&:to_i)

abm = m.times.map { gets.chomp.split(" ").map(&:to_i) }
cdm = m.times.map { gets.chomp.split(" ").map(&:to_i) }

table1 = Array.new(n) { Array.new(n) { nil } }
table2 = Array.new(n) { Array.new(n) { nil } }

abm.each {|a, b|
  table1[a-1][b-1] = true
  table1[b-1][a-1] = true
}
cdm.each {|a, b|
  table2[a-1][b-1] = true
  table2[b-1][a-1] = true
}

ans = true

(0...n).to_a.permutation.each { |per|
  ok = true
  (0...n).each { |i|
    (0...n).each { |j|
      s = per[i]
      t = per[j]
      ok = false if table1[i][j] != table2[s][t]
    }
  }
  if ok
    puts "Yes"
    exit
  end
}
puts "No"
