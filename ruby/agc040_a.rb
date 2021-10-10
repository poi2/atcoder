# https://atcoder.jp/contests/agc040/tasks/agc040_a

def solve(str)
  str.scan(/(<*)(>*)/)
    .inject(0) { |sum, (ge, le)|
      gl, ll = ge.length, le.length
      sum += (gl * (gl - 1) / 2) + (ll * (ll - 1) / 2) + [gl, ll].max
    }
end

str = gets.chomp
puts solve(str)

# <>>
# 0<2>1>0
# <<>>>
# 0<1<3>2>1>0

# 1>0<1<2<3
