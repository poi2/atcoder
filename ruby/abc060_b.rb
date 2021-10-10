# https://atcoder.jp/contests/abc060/tasks/abc060_b

a, b, c = gets.chomp.split(" ").map(&:to_i)

sum = 0
memo = []
while true
  sum += a
  rest = sum % b
  if rest == c
    puts "YES"
    exit
  else
    if memo.include? rest
      puts "NO"
      exit
    else
      memo.push(rest)
    end
  end
end
