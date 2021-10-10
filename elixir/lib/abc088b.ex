require Integer

defmodule Main do
  def main do
    _ = IO.read(:line)
    cards = IO.read(:line)
      |> String.trim
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort
      |> Enum.reverse

    acc = %{ 0 => 0, 1 => 0 }
    acc = solve(cards, acc, 0)
    IO.puts(acc[0] - acc[1])
  end

  def solve([head], acc, i) do
    acc = Map.put(acc, i, acc[i] + head)
    acc
  end

  def solve([head | tail], acc, i) do
    acc = Map.put(acc, i, acc[i] + head)
    solve(tail, acc, rem(i + 1, 2))
  end
end

defmodule Main do
  def main do
    _ = IO.read(:line)
    cards = IO.read(:line)
      |> String.trim
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort
      |> Enum.reverse

    solve(cards, 0, 0) |> IO.puts
  end

  def solve([alice], sum_alice, sum_bob) do
    (sum_alice + alice) - sum_bob
  end

  def solve([alice, bob], sum_alice, sum_bob) do
    (sum_alice + alice) - (sum_bob + bob)
  end

  def solve([alice, bob | tail], sum_alice, sum_bob) do
    solve(tail, sum_alice + alice, sum_bob + bob)
  end
end

defmodule Main do
  def main do
    _ = IO.read(:line)
    cards = IO.read(:line)
      |> String.trim
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort
      |> Enum.reverse

    alice = cards
      |> Enum.with_index
      |> Enum.filter(fn({x, i}) -> rem(i, 2) == 0 end)
      |> Enum.map(fn({x, _}) -> x end)
      |> Enum.sum

    bob = cards
      |> Enum.with_index
      |> Enum.filter(fn({x, i}) -> rem(i, 2) == 1 end)
      |> Enum.map(fn({x, _}) -> x end)
      |> Enum.sum

    alice - bob |> IO.puts
  end
end
