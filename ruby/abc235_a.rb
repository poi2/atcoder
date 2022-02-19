a, b, c = gets.chomp.split("").map(&:to_i)

def f(a, b, c)
  a * 100 + b * 10 + c
end

puts f(a, b, c) + f(b, c, a) + f(c, a, b)
