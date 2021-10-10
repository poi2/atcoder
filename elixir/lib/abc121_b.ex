require Integer

defmodule Main do
  def main do
    [n, m, c] = IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)
    bs = IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)

    list_as =
      for i <- 1..n do
        IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)
      end

    list_as |> Enum.map(fn(as) -> solve(as, bs, m, c) end) |> Enum.sum |> IO.puts
  end

  def solve(as, bs, m, c) do
    ans = (0..m-1) |> Enum.map(fn(i) -> Enum.at(as, i) * Enum.at(bs, i) end) |> Enum.sum
    if (ans + c) > 0 do
      1
    else
      0
    end
  end
end
