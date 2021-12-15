class UnionFind
  attr_accessor :parent
  def initialize(size)
    @parent = Array.new(size, -1)
  end

  # xが所属するグループのルートノードを返す
  def root(x)
    if @parent[x] < 0
      x
    else
      # 経路圧縮を行なう
      @parent[x] = root(@parent[x])
    end
  end

  # xとyが同じグループに属しているか判定する
  def same_parent?(x, y)
    root(x) == root(y)
  end

  # xとyが所属するグループを併合する
  def merge(x, y)
    x_root = root(x)
    y_root = root(y)

    return false if x_root == y_root

    # merge technique
    if @parent[x_root] > @parent[y_root]
        x_root, y_root = y_root, x_root
    end

    @parent[x_root] += @parent[y_root]
    @parent[y_root] = x_root

    return true
  end

  # xが所属するグループのサイズを返す
  def size(x)
    return -@parent[root(x)]
  end
end

n, m = gets.chomp.split(" ").map(&:to_i)

used = Array.new(n + 1) { 0 }
uf = UnionFind.new(n + 1)
m.times {
  a, b = gets.chomp.split(" ").map(&:to_i)
  if uf.same_parent?(a, b)
    puts "No"
    exit
  end
  uf.merge(a, b)
  used[a] += 1
  used[b] += 1
}
if used.find{|v| v > 2 }
  puts "No"
  exit
end

puts "Yes"

