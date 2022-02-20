require 'prime'
a, b, c, d = gets.split(" ").map(&:to_i)

primes = (1..200).select{|i| Prime.prime?(i) }

ans = "Aoki"
(a..b).each { |i|
  if (c..d).all? { |j| !primes.include?(i + j) }
    ans = "Takahashi"
  end
}
puts ans
