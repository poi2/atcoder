# https://atcoder.jp/contests/agc015/tasks/agc015_a

N, A, B = gets.chomp.split(" ").map(&:to_i)

def solve
  case
  when A > B
    0
  when N == 1 && A == B
    1
  when N == 1 && A != B
    0
  else
    min = A * (N - 1) + B
    max = B * (N - 1) + A
    max - min + 1
  end
end

puts solve
