require Integer

# https://atcoder.jp/contests/abc211/tasks/abc211_d

defmodule Main do
  def main do
    [n, m] = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)
    nodes = (1..m) |> Enum.reduce(%{}, fn(_, acc) ->
      [a, b] = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)
      as = Map.get(acc, a, []) ++ [b]
      bs = Map.get(acc, b, []) ++ [a]
      acc |> Map.put(a, as) |> Map.put(b, bs)
    end)
    IO.inspect nodes

    solve(nodes, %{ :dis => %{}, :num => %{}, :que => [1] }) |> Map.get(n) |> IO.puts
  end

  def solve(nodes, old_acc) do
    que = old_acc |> Map.get(:que)
    case que do
      [h|t] ->
        # NOTE reduce 不要
        Enum.reduce(old_acc, fn(i, acc) ->
          dis = acc |> Map.get(:dis)
          num = acc |> Map.get(:num)
          que = acc |> Map.get(:que)
          cur_dis = Map.get(dis, i, nil)
          [new_dis, new_num, new_que] =
            cond do
              cur_dis == nil ->
                new_dis = Map.get(dis, h) + 1
                new_num  = Map.get(num, i) + Map.get(num, h)
                [new_dis, new_num, i]
              cur_dis == (Map.get(dis, h) + 1) ->
                new_num  = Map.get(num, i) + Map.get(num, h)
                [cur_dis, new_num, nil]
            end
          acc
            |> Map.put(:dis, (dis |> Map.put(i, new_dis)))
            |> Map.put(:num, (num |> Map.put(i, new_num)))

          if new_que == nil do
            acc
          else
            acc |> Map.put(:que, (que |> Map.put(i, new_que)))
          end
          solve(nodes, acc)
        end)
      [] -> old_acc |> Map.get(:num)
    end
  end

  def solvep(dic, i, j) do
    cdis, cnum, ndis, nnum
    cond do
      ndis == nil  -> [cdis + 1, cnum,      , n]
      ndis == cdis -> [cdis,     cnum + nnum, nil]
      false        -> [ndis,     nnum,      , nil]
    end
  end
end

{
  que: [],
  1: {dis:, num:},
  2: {dis:, num:},
}
