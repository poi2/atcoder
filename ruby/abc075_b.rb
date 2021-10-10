# https://atcoder.jp/contests/abc075/tasks/abc075_b

h, w = gets.chomp.split(" ").map(&:to_i)
list = h.times.map { gets.chomp.chars }

def new_list(list)
  tmp = list.map{|l| ['.'] + l + ['.'] }
  len = tmp.first.length
  [Array.new(len) {'x'}] + tmp + [Array.new(len) {'x'}]
end

new_list = new_list(list)

puts (1..h).map { |i|
  (1..w).map { |j|
    if new_list[i][j] == '#'
      '#'
    else
      [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]].sum { |x, y|
        new_list[i+x][j+y] == '#' ? 1 : 0
      }.to_s
    end
  }.join
}.join("\n")
