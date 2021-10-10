# https://atcoder.jp/contests/abc096/tasks/abc096_c

h, w = gets.chomp.split.map(&:to_i)

list =
  [['.'] * (w + 2)] +
  h.times.map {
    ['.'] + gets.chomp.chars + ['.']
  } +
  [['.'] * (w + 2)]

(1..h).each { |i|
  (1..w).each { |j|
    next if list[i][j] == '.'

    if [[-1, 0], [0, -1], [0, 1], [1, 0]].none? { |s, t| list[i+s][j+t] == '#' }
      puts 'No'
      exit
    end
  }
}
puts 'Yes'
