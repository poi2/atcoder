require Integer

defmodule Main do
  def main do
    [n, yen] =
      IO.read(:line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    solve(n, yen) |> IO.puts
  end

  def solve(n, yen) do
    try do
      for x <- 0..n do
        for y <- 0..(n - x) do
          z = n - x - y
          if x * 10000 + y * 5000 + z * 1000 == yen do
            throw([x, y, z])
          end
        end
      end
      "-1 -1 -1"
    catch
      [x, y, z] -> "#{x} #{y} #{z}"
    end
  end
end
