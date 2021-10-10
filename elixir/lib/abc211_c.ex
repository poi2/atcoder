require Integer

# https://atcoder.jp/contests/abc211/tasks/abc211_c

defmodule Main do
  def main do
    list = IO.read(:line) |> String.trim |> String.split("", trim: true)

    solve(list, 0, acc_init()) |> IO.puts
  end

  def at(s) do
    # "chokudai" |> String.split("", trim: true) |> Enum.with_index |> Enum.reduce(%{}, fn({k, i}, acc) -> Map.put(acc, k, i) end)
    %{ "a" => 6, "c" => 0, "d" => 5, "h" => 1, "i" => 7, "k" => 3, "o" => 2, "u" => 4 } |> Map.get(s)
  end

  def acc_init() do
    (0..7)|>Enum.reduce(%{}, fn(i, acc) -> acc |> Map.put(i, 0) end)
  end

  def solve([], _, acc) do
    div = :math.pow(10, 9) + 7 |> trunc
    acc |> Map.get(7) |> rem(div)
  end


  def solve([head|tail], i, old_acc) do
    cursol = head |> at
    new_acc =
      (0..7) |> Enum.reduce(%{}, fn(j, acc) ->
        count =
          if j == cursol do
            Map.get(old_acc, j) + Map.get(acc, j-1, 0)
          else
            Map.get(old_acc, j)
          end
        count =
          if cursol == 0 && j == 0 do
            count + 1
          else
            count
          end
        Map.put(acc, j, count)
      end)
    solve(tail, i + 1, new_acc)
  end
end
