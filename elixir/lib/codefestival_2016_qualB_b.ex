require Integer

defmodule Main do
  def main do
    [_, a, b] = IO.read(:line) |> String.trim() |> String.split(" ") |> Enum.map(&String.to_integer/1)
    list = IO.read(:line) |> String.trim() |> String.split("", trim: true)

    solve(list, 0, 0, a, b)
  end

  def solve([], _, _, _, _) do
  end

  def solve([head] , total_passed, b_passed, a, b) do
    case head do
      "a" ->
        if total_passed < a + b do
          IO.puts "Yes"
        else
          IO.puts "No"
        end
      "b" ->
        if total_passed < (a + b) && b_passed < b do
          IO.puts "Yes"
        else
          IO.puts "No"
        end
      "c" ->
        IO.puts "No"
    end
  end

  def solve([head|tail] , total_passed, b_passed, a, b) do
    case head do
      "a" ->
        if total_passed < a + b do
          IO.puts "Yes"
          solve(tail, total_passed + 1, b_passed, a, b)
        else
          IO.puts "No"
          solve(tail, total_passed, b_passed, a, b)
        end
      "b" ->
        if total_passed < a + b && b_passed < b do
          IO.puts "Yes"
          solve(tail, total_passed + 1, b_passed + 1, a, b)
        else
          IO.puts "No"
          solve(tail, total_passed, b_passed, a, b)
        end
      "c" ->
        IO.puts "No"
        solve(tail, total_passed, b_passed, a, b)
    end
  end
end
