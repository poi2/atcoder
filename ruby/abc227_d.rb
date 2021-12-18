_, k = gets.chomp.split(" ").map(&:to_i)
an = gets.chomp.split(" ").map(&:to_i).sort

up = 2 ** 64
lo = 0

while (up - lo) > 1 do
  sum = 0
  i = (up + lo) / 2
  an.each { |a| sum += [a, i].min }
  if sum >= k * i
    lo = i
  else
    up = i
  end
end

puts lo
