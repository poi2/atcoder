# https://atcoder.jp/contests/abc107/tasks/abc107_b

h, w = gets.chomp.split.map(&:to_i)
list = h.times.map { gets.chomp.chars }

def reject_row(list)
  list.reject { |l| l.all?{ |i| i == '.' } }
end

def reject_col(old_list)
  reject_row(old_list.transpose)
end

def solve(old_list)
  list = reject_row(old_list)
  reject_col(list).transpose.map{ |l| l.join }.join("\n")
end

puts solve(list)
