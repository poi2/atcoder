# https://atcoder.jp/contests/abc213/tasks/abc213_c

H, W, N = gets.chomp.split(" ").map(&:to_i)
list =  N.times.map { |i| gets.chomp.split(" ").map(&:to_i) + [i+1] }

as, bs = [], []
list.each { |a, b, c| as << a; bs << b }

amemo={}; as.sort.uniq.each_with_index{|n, i| amemo[n] == nil ? amemo[n] = i : nil }
bmemo={}; bs.sort.uniq.each_with_index{|n, i| bmemo[n] == nil ? bmemo[n] = i : nil }

list.each {|x, y, i|
  puts "#{amemo[x]+1} #{bmemo[y]+1}"
}
