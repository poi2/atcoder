# https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_c

# n = gets.chomp.to_i
# points = n.times.map { gets.chomp.split(" ").map(&:to_i) }

class P
  attr :x, :y
  def initialize(x, y)
    @x, @y = x, y
  end

  def -(another)
    new_x = self.x - another.x
    new_y = self.y - another.y
    P.new(new_x, new_y)
  end

  def +(another)
    new_x = self.x + another.x
    new_y = self.y + another.y
    P.new(new_x, new_y)
  end

  def ==(another)
    self.x == another.x && self.y == another.y
  end

  def others(another)
    dx = another.x - self.x
    dy = another.y - self.y

    p11 = another + P.new(dy, -dx)
    p12 = self + P.new(dy, -dx)

    # mid = P.new(self + P.new(dx/2.0, dy/2.0))
    # mid_dx = mid.x - self.x
    # mid_dy = mid.y - self.y
    # p21 = P.new(mid + P.new(mid_dy, -mid_dx))
    # p22 = P.new(mid + P.new(-mid_dy, mid_dx))

    # p31 = P.new(self + P.new(-dy, dx))
    # p32 = P.new(another + P.new(-dy, dx))
    [
      [p11, p12],
      # [p21, p22],
      # [p31, p32]
    ]
  end
end
p1 = P.new(1,1)
p2 = P.new(2,4)
p p1.others(p2)
