H, W, k = gets.chomp.split(" ").map(&:to_i)
x1, y1, x2, y2  = gets.chomp.split(" ").map(&:to_i)

DIV = 998244353

# def dp(count, naname, tate, yoko, on)
#   if count == 0
#     return on % DIV
#   end

#   next_tate = (tate * (H - 2) + naname + on * (H - 1)) % DIV
#   next_yoko = (yoko * (H - 2) + naname + on * (W - 1)) % DIV
#   next_naname = (tate * (W - 1) + yoko * (H - 1) + naname * (W + H - 4)) % DIV
#   next_on = (tate + yoko) % DIV
#   dp(count - 1, next_naname, next_tate, next_yoko, next_on)
# end

naname, tate, yoko, on = 0, 0, 0, 0
if x1 != x2 && y1 != y2
  naname = 1
elsif x1 == x2 && y1 != y2
  yoko = 1
elsif x1 != x2 && y1 == y2
  tate = 1
else
  on = 1
end

# puts dp(k, naname, tate, yoko, on)

k.times {
  next_tate = (tate * (H - 2) + naname + on * (H - 1)) % DIV
  next_yoko = (yoko * (W - 2) + naname + on * (W - 1)) % DIV
  next_naname = (tate * (W - 1) + yoko * (H - 1) + naname * (W + H - 4)) % DIV
  next_on = (tate + yoko) % DIV

  naname = next_naname
  tate = next_tate
  yoko = next_yoko
  on = next_on
}
puts on % DIV
