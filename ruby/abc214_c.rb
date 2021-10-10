# https://atcoder.jp/contests/abc214/tasks/abc214_c

N = gets.chomp.to_i
sn = gets.chomp.split(" ").map(&:to_i)
tn = gets.chomp.split(" ").map(&:to_i)

# N = 3
# sn = "4 1 5".chomp.split(" ").map(&:to_i)
# tn = "3 5 100".chomp.split(" ").map(&:to_i)

# N = 4
# sn = "100 100 100 100".chomp.split(" ").map(&:to_i)
# tn = "1 1 1 1".chomp.split(" ").map(&:to_i)

INF = 1000000001
memo = Array.new(N) {INF}
used = {}

sorted_tn = tn.map.with_index{|n, i| [n, i]}.sort
sorted_tn.each { |t, n|
  next if used[n] == true
  used[n] = true

  memo[n] = t < memo[n] ? t : memo[n]

  i = 1
  while used[n+i] == nil
    bef_i = (N + n + i - 1) % N
    cur_i = (N + n + i) % N

    pass_t = memo[bef_i] + sn[bef_i]
    taka_t = tn[cur_i]
    case
    when pass_t <= taka_t
      used[cur_i] = true
      memo[cur_i] = pass_t
      i = (i+1) % N
    when pass_t > taka_t
      break
    end
  end
}

puts memo
