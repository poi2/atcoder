require Integer

defmodule Main do
  def main do
    list = IO.read(:line) |> String.trim |> String.split("", trim: true)

    solve(list, 0, 0) |> IO.puts
  end

  def solve([], _, total_cost) do
    total_cost
  end

  def solve([i|tail], cost, total_cost) do
    case i do
      "W" -> solve(tail, cost    , total_cost + cost)
      "B" -> solve(tail, cost + 1, total_cost       )
    end
  end
end

# NOTE
# バブルソートの交換回数を計算する
