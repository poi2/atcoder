require Integer

# https://atcoder.jp/contests/agc040/tasks/agc040_a

defmodule Main do
  def main do
    list = IO.read(:line) |> String.trim |> String.split("", trim: true)

    solve(list, nil, 0, 0) |> IO.puts
  end

  # def solve([], _, _, sum) do
  #   sum
  # end

  # def solve([head|tail], before, counter, sum) do
  #   case before == head do
  #     true ->
  #       solve(tail, head, counter + 1, sum)
  #     false ->
  #       solve(tail, head, 1, sum + counter)
  #   end
  # end
end



# 0<3>2>1>0<1<2>0<1<2<3<4<5>2>1>0<1
# 
# < -> 1: (1..0).sum
# > -> 3: (0..3).sum
# < -> 2: (1..2).sum
# > -> 1: (0..0).sum
# < -> 5: (1..5).sum
# > -> 3: (0..2).sum
# < -> 1: (1..1).sum

