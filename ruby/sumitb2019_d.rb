# https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d

_ = gets.chomp.to_i
s = gets.chomp

ans = 0
('000'..'999').each { |str|
  i, j, k = str.chars
  if (a = s.index(i, 0)) && (b = s.index(j, a + 1)) && s.index(k, b + 1)
    ans += 1
  end
}
puts ans
