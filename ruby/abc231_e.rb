def solve(old_rem, an, i, count)
  div = old_rem / an[i]
  count1 = div
  count2 = div + 1
  rem1 = old_rem - count1 * an[i]
  rem2 = count2 * an[i] - old_rem

  if i > 0
    case [rem1, rem2]
    in [0, 0]
      return [count + count1, count + count2]
    in [0, _]
      return [count + count1, solve(rem2, an, i - 1, count + count2)]
    in [_, 0]
      return [solve(rem1, an, i - 1, count + count1), count + count2]
    in [_, _]
      return [solve(rem1, an, i - 1, count + count1), solve(rem2, an, i - 1, count + count2)]
    end
  else
    return [count + count1]
  end
end

n, x = gets.chomp.split(" ").map(&:to_i)
an = gets.chomp.split(" ").map(&:to_i)

i = an.bsearch_index { |i| i >= x } || (n - 1)

p solve(x, an, i, 0).flatten.min
