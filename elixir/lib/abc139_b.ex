require Integer

defmodule Main do
  def main do
    [a, b] =
      IO.read(:line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    solve(a, b) |> IO.puts
  end

  def solve(a, b) do
    case b do
      1 -> 0
      _ ->
        res = (b - 1) / (a - 1)
        if res == trunc(res) do
          trunc(res)
        else
          trunc(res) + 1
        end
    end
  end
end
