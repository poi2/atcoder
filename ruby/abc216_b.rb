# https://atcoder.jp/contests/abc216/tasks/abc216_b

n = gets.chomp.to_i
st = n.times.map { gets.chomp }
puts st.size != st.uniq.size ? "Yes" : "No"
