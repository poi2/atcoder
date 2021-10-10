defmodule Main do
  def main() do
    n = IO.read(:line) |> String.trim |> String.to_integer
    [h0, h1 | _] = h = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)

    dp = %{ 0 => 0, 1 => abs(h1 - h0)}

    solve(dp, 2, h) |> Map.get(n - 1) |> IO.puts
  end

  def solve(dp, _, [_, _]) do
    dp
  end

  def solve(dp, idx, [h0, h1, h2]) do
    dp
    |> Map.put(idx, min(dp[idx - 1] + abs(h2 - h1), dp[idx - 2] + abs(h2 - h0)))
  end

  def solve(dp, idx, [h0, h1, h2 | tail]) do
    dp
    |> Map.put(idx, min(dp[idx - 1] + abs(h2 - h1), dp[idx - 2] + abs(h2 - h0)))
    |> solve(idx + 1, [h1, h2 | tail])
  end
end

Main.main
