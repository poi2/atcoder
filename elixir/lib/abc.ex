require Integer

defmodule Abc do
  def main() do
  end

  def solve_086a() do
    # [a, b] =
    #   IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)
    [a, b] = [3, 5]

    case rem(a * b, 2) do
      0 -> "Even"
      1 -> "Odd"
    end |> IO.puts()
  end

  def solve_081a() do
    s = "010"
    s
      |> String.codepoints()
      |> Enum.count(fn x -> x == "1" end)
      |> IO.puts
  end

  def solve_081b() do
    s = "32 16 64"
    list = s
      |> String.split(" ")
      |> Enum.map(&String.to_integer/1)

    solve_081b(list, 0)
      |> IO.puts()
  end

  def solve_081b(list, count) do
    next_list = Enum.map(list, &(div(&1, 2)))
    case Enum.all?(next_list, &Integer.is_even/1) do
      true -> solve_081b(next_list, count + 1)
      false -> count
    end
  end

  def solve_087b() do
    # [a, b, c, total] = [2, 2, 2, 100]
    [a, b, c, total] = [30, 40, 50,   6000]
    for i <- 0..a,
        j <- 0..b,
        k <- 0..c do
      500 * i + 100 * j + 50 * k
    end
    |> Enum.count(&(&1 == total))
    |> IO.puts
  end

  def solve_083b() do
    [n, a, b] = [20, 2, 5]
    1..n
    |> Enum.filter(
      fn i ->
        sum = Integer.digits(i) |> Enum.sum
        a <= sum && sum <= b
      end
    )
    |> Enum.sum
    |> IO.puts
  end

  def solve_083b2() do
    [n, a, b] = [20, 2, 5]

    solve_083b2(n, a, b, 0)
    |> IO.puts
  end

  def solve_083b2(0, _, _, sum) do
    sum
  end

  def solve_083b2(n, a, b, sum) when n > 0 do
    x = Integer.digits(n) |> Enum.sum
    # IO.inspect([n, Integer.digits(n), x])
    case a <= x && x <= b do
      true  -> solve_083b2(n - 1, a, b, sum + x)
      false -> solve_083b2(n - 1, a, b, sum)
    end
  end

  def solve_088b do
    [_ | l] = [4, 20, 18, 2, 18]
    sorted = Enum.sort(l, :desc)
    a = sorted |> Enum.with_index |> Enum.filter(fn(x) -> elem(x, 1) |> Integer.is_even end) |> Enum.map(fn(x)-> elem(x, 0) end) |> Enum.sum
    b = sorted |> Enum.with_index |> Enum.filter(fn(x) -> elem(x, 1) |> Integer.is_odd  end) |> Enum.map(fn(x)-> elem(x, 0) end) |> Enum.sum

    a - b |> IO.puts
  end

  def solve_085b do
    [_ | l] = [4, 5, 5, 4, 8]
    l |> Enum.uniq |> Enum.count |> IO.puts
  end

  def solve_085c do
    [n, y] = [9, 450001]
    for i <- 0..n, j <- 0..(n-i) do
      { i, j, n - i - j }
    end
    |> Stream.filter(fn({ i, j, k }) -> 10000 * i + 5000 * j + 1000 * k == y end)
    |> Enum.take(1)
    |> Enum.at(0, {-1, -1, -1})
    |> IO.inspect
  end

  def solve_049c do
    str_list = ["dreamerasererasedreamereraser", "dreamerer"]

    str_list |> Enum.map(&(Abc.solve_049c(&1))) |> IO.inspect
  end

  def solve_049c(str) do
    cond do
      Regex.match?(~r/dream$/,   str) -> solve_049c(String.split_at(str, -5) |> elem(0))
      Regex.match?(~r/dreamer$/, str) -> solve_049c(String.split_at(str, -7) |> elem(0))
      Regex.match?(~r/erase$/,   str) -> solve_049c(String.split_at(str, -5) |> elem(0))
      Regex.match?(~r/eraser$/,  str) -> solve_049c(String.split_at(str, -6) |> elem(0))
      str == "" -> :YES
      true -> :NO
    end
  end
end

Abc.main
Abc.solve_086a
Abc.solve_081a
Abc.solve_081b
Abc.solve_087b
Abc.solve_083b
Abc.solve_083b2
Abc.solve_088b
Abc.solve_085b
Abc.solve_085c
Abc.solve_049c
