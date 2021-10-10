defmodule Main do
  def main() do
    s = IO.read(:line) |> String.trim() |> String.reverse
    matchers = ["dream", "dreamer", "erase", "eraser"] |> Enum.map(&String.reverse/1)

    solve(s, matchers) |> IO.puts
  end

  def solve("", matchers) do
    "YES"
  end

  def solve(str, matchers) do
    new_str = replace(str, matchers)
    if str == new_str do
      "NO"
    else
      solve(new_str, matchers)
    end
  end

  def replace(str, [head]) do
    String.replace_prefix(str, head, "")
  end

  def replace(str, [head|tail]) do
    replace(String.replace_prefix(str, head, ""), tail)
  end
end
