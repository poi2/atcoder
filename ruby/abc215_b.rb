# https://atcoder.jp/contests/abc215/tasks/abc215_b

# log2 10 = 3.33
# log2 10**18 = 18 * log 2 10 = 18 * 3.33 = 59.94

n = gets.chomp.to_i
(0..60).each do |i|
  if 2 ** i > n
    puts i - 1
    exit
  end
end
