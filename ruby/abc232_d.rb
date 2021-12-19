H, W = gets.chomp.split(" ").map(&:to_i)
field = H.times.map { gets.chomp.split("") }
used = Array.new(H) { Array.new(W) {nil} }

def bfs(i, j, field, used)
  if i < H && j < W && used[i][j].nil? && field[i][j] == '.'
    used[i][j] = i + j

    bfs(i + 1, j, field, used)
    bfs(i, j + 1, field, used)
  end
end

bfs(0, 0, field, used)

p used.map{|l| l.compact.max}.compact.max.to_i + 1
