q = gets.chomp.to_i
n = 1048576

used = {}

q.times.map {
  t, x = gets.chomp.split(" ").map(&:to_i)

  if t == 1
    h = x
    mod = h % n
    while used[mod] != nil
      h += 1
      mod = h % n
    end
    used[mod] = x
  else
    puts used[x % n] || -1
  end
}
