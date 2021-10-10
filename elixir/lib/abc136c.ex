require Integer

defmodule Main do
  def main do
    n = IO.read(:line) |> String.trim |> String.to_integer
    h = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)

    solve(h, -1, n) |> IO.puts
  end

  def solve([_], _, n) when n < 0 do
    'No'
  end

  def solve([], _, _) do
    'Yes'
  end

  def solve([head], max, n) do
    cond do
      head >= max -> solve([], head, n)
      max == head + 1 -> solve([], max, n - 1)
      max > head + 1 -> 'No'
    end
  end

  def solve([head | tail], max, n) do
    cond do
      head >= max -> solve(tail, head, n)
      max == head + 1 -> solve(tail, max, n - 1)
      max > head + 1 -> 'No'
    end
  end
end
