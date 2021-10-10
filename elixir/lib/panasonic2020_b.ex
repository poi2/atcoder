require Integer

defmodule Main do
  def main do
    [n, w] = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer/1)

    solve(n, w) |> IO.puts
  end

  def solve(1, _) do
    1
  end

  def solve(_, 1) do
    1
  end

  def solve(n, w) do
    width = w / 2 |> trunc
    if rem(w, 2) == 0 do
      width * n
    else
      width * n + trunc((n + 1) / 2)
    end
  end
end
