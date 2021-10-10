defmodule Main do
  def main() do
    n = IO.read(:line) |> String.trim |> String.to_integer

    i = 0
    acc = %{ 0 => %{a: 0, b: 0, c: 0} }
    [a, b, c] = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)

    current_dp = acc[i]
    next_dp = %{
      a: max(current_dp[:b] + a, current_dp[:c] + a),
      b: max(current_dp[:c] + b, current_dp[:a] + b),
      c: max(current_dp[:a] + c, current_dp[:b] + c),
    }
    Map.put(acc, i + 1, next_dp)
  end

  def solve(n, 0, _) do
    solve(n, 1, %{ 0 => %{a: 0, b: 0, c: 0} })
  end

  def solve(n, n, acc) do
    acc[n] |> Map.values |> Enum.max
  end

  def solve(n, i, acc) do

  end
end
