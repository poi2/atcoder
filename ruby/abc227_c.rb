def solve(max, min)
  lim = Math.sqrt(max).to_i
  ans = 0
  (min..lim).each { |i|
    ans += (max / i) - (i - 1)
  }
  ans
end

n = gets.chomp.to_i
total = 0
(1..n).each { |a|
  ans = solve(n / a, a)
  if ans > 0
    total += ans
  else
    break
  end
}
puts total