require Integer

defmodule Main do
  def main do
    n = IO.read(:line) |> String.trim() |> String.to_integer

    solve(n) |> IO.puts
  end

  def solve(n) do
    tmp = n / tax

    cal1 = trunc(tmp)
    ans1 = cal1 * tax |> trunc

    cal2 = ceil(tmp)
    ans2 = cal2  * tax |> trunc

    # case n do
    #   ans1 -> cal1
    #   ans2 -> cal2
    #   _ -> ":("
    # end

    cond do
      ans1 == n -> cal1
      ans2 == n -> cal2
      true -> ":("
    end

  end

  def tax do
    1.08
  end
end
