require Integer

defmodule Main do
  def main do
    n = IO.read(:line)
    xs = IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)

    max = xs |> Enum.max
    (1..max) |> Enum.map(fn(x) -> calc(x, xs) end) |> Enum.min |> IO.puts
  end

  def calc(p, xs) do
    xs |> Enum.map( fn(x) -> (x - p) |> :math.pow(2) end ) |> Enum.sum |> trunc
  end
end
