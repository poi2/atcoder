require Integer

defmodule Main do
  def main do
    input =
      IO.binstream(:stdio, :line)
      |> Stream.map(fn line ->
        line
        |> String.trim
        |> String.to_integer
      end)

    n = Enum.at(input, 0)
    an = Stream.take(input, n)
      |> Enum.with_index(1)
      |> Enum.map(fn({n, i}) -> {i, n} end)
      |> Enum.to_list
      |> Map.new

    solve(1, an, 0) |> IO.puts
  end

  def solve(2, _, count) do
    count
  end

  def solve(nil, _, _) do
    -1
  end

  def solve(i, an, count) do
    case Map.get_and_update(an, i, fn(cv) -> { cv, nil } end) do
      { next, an } -> solve(next, an, count + 1)
    end
  end
end

# defmodule WelcomeToAtCoder do
#   def main do
#     a = IO.read(:line) |> String.trim |> String.to_integer
#     [b, c] = IO.read(:line) |> String.trim |> String.split(" ") |> Enum.map(&String.to_integer(&1))
#     s = IO.read(:line)

#     "#{a + b + c} #{s}" |> IO.puts
#   end
# end

Main.main
