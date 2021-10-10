# https://atcoder.jp/contests/abc214/tasks/abc214_d
# UnionFind 木

# https://atcoder.jp/contests/abc214/submissions/25058893
class UnionFind
  def initialize(n)
    @parent = Array.new(n, -1)
  end

  # NOTE
  # `root': undefined method `<' for nil:NilClass (NoMethodError)
  # が発生した場合、初期化 n を小さく指定している可能性を考える
  def root(x)
    @parent[x] < 0 ? x : (@parent[x] = root(@parent[x]))
  end

  def size(x)
    -@parent[root(x)]
  end

  def unite(x, y)
    x = root(x)
    y = root(y)
    return if x == y

    x, y = y, x if size(x) < size(y)
    @parent[x] -= size(y)
    @parent[y] = x
  end
end

n = gets.chomp.to_i
paths = (n-1).times.map{
  u, v, w = gets.chomp.split(" ").map(&:to_i)
  [u-1, v-1, w]
}.sort_by { |_, _, w| w }

uf = UnionFind.new(n)

puts paths.inject(0) { |acc, (u, v, w)|
  cost = w * uf.size(u) * uf.size(v)
  uf.unite(u, v)
  acc = acc + cost
}
