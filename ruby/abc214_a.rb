# https://atcoder.jp/contests/abc214/tasks/abc214_a

n = gets.chomp.to_i

def num(n)
  case n
  when (1..125)
    4
  when (126..211)
    6
  when (212..214)
    8
  end
end

puts num(n)
