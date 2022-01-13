n, k = gets.chomp.split(" ").map(&:to_i)
pn = gets.chomp.split(" ").map(&:to_i)

sorted = pn.first(k).sort
sikii = sorted.first

ans = []

ans << sikii
(k..n-1).each { |i|
  pni = pn[i]

  if sikii < pni
    sorted.delete_at(0)
    idx = sorted.bsearch_index{ |x| x >= pni } || -1
    sorted.insert(idx, pni)
    sikii = sorted.first
  end
  ans << sikii
}
puts ans
