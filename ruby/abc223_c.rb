n = gets.chomp.to_i
abc = n.times.map {
  a, b = gets.chomp.split(" ").map(&:to_i)
  [a, b, a/(b.to_f)]
}
total = abc.sum{ |_, _, c| c }
half = total / 2.0

dist = 0
n.times { |i|
  if half - abc[i][2] >= 0
    half -= abc[i][2]
    dist += abc[i][0]
  else
    dist += half * abc[i][1]
    puts dist
    break
  end
}
