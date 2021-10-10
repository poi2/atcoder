# https://atcoder.jp/contests/abc213/tasks/abc213_e

H, W = gets.chomp.split(" ").map(&:to_i)
Sij = H.times.map{ gets.chomp.split("") }

def infield?(i, j)
  return false unless i >= 0 && i < H
  return false unless j >= 0 && j < W
  true
end

D4 = [[-1, 0], [1, 0], [0, -1], [0, 1]]
D21 =
  (-2..2).flat_map { |i|
    (-2..2).map { |j|
      [i, j]
    }
  }.reject { |i, j| i.abs + j.abs > 3 }
INF = 2**32

def solve
  que = [[0,0]]
  dist_table = Array.new(H) { Array.new(W, INF) }
  dist_table[0][0] = 0
  used = Array.new(H) { Array.new(W) }

  while que.size > 0
    i, j = que.shift
    next if used[i][j]
    used[i][j] = true

    dist = dist_table[i][j]
    D4.each { |di, dj|
      x, y = i + di, j + dj
      next unless infield?(x, y)
      next if Sij[x][y] == '#'
      next if dist_table[x][y] <= dist
      dist_table[x][y] = dist
      que.unshift([x, y])
    }
    D21.each { |di, dj|
      x, y = i + di, j + dj
      next unless infield?(x, y)
      next if dist_table[x][y] <= dist + 1
      dist_table[x][y] = dist + 1
      que.push([x, y])
    }
  end
  dist_table
end

puts solve[H-1][W-1]
