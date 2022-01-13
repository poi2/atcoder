x = gets.chomp.to_i

if x < 10
  puts x
  exit
end

list = x.to_s.split("").map(&:to_i)
flg = list.each_cons(2).map {|i,j| j - i}.uniq.count == 1

len = list.size
x1, x2 = list.first(2)

def zen(x1, x2, len)
  d = x2 - x1
  ([x1, x2] + (len-2).times.map { |i| x2 + (i+1) * d }).join.to_i
end

def in?(x1, x2, len)
  a = (x2 - x1) * (len - 1) + x1
  if 0 <= a && a <= 9
    true
  else
    false
  end
end

until in?(x1, x2, len) && zen(x1, x2, len) >= x
  if x2 < 9
    x2 += 1
  else
    if x1 < 9
      x1 += 1
      x2 = 0
    else
      x1 = 1
      x2 = 0
      len += 1
    end
  end
end
puts zen(x1, x2, len)