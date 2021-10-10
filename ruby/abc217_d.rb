# https://atcoder.jp/contests/abc217/tasks/abc217_d

require 'set'

l, q = gets.chomp.split(" ").map(&:to_i)
list = []
ans = []

q.times {
  c, x = gets.chomp.split(" ").map(&:to_i)
  if c == 1
    idx = list.bsearch_index { |i| i > x }
    if idx == nil
      list << x
    else
      list.insert(idx, x)
    end
  else
    left_idx = list.bsearch_index { |i| i > x }
    if left_idx == nil
      left = l
      right = list[-1] || 0
    else
      left  = list[left_idx]
      right = if left_idx == 0
        0
      else
        list[left_idx-1]
      end
    end

    ans << (left - right)
  end
}
puts ans
