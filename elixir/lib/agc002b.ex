require Integer

defmodule Main do
  def main() do
    [a, b] =
      IO.read(:line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    solve(a, b) |> IO.puts()
  end

  def solve(_, 0) do
    'Zero'
  end

  def solve(0, _) do
    'Zero'
  end

  def solve(a, b) when a < 0 and b > 0 do
    'Zero'
  end

  def solve(a, b) when a > 0 do
    'Positive'
  end

  def solve(a, b) do
    case rem(b - a, 2) do
      0 -> 'Negative'
      _ -> 'Positive'
    end
  end
end
