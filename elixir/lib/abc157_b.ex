require Integer

defmodule Main do
  def main do
    a1 = IO.read(:line) |> String.trim |> String.split(" ")
    a2 = IO.read(:line) |> String.trim |> String.split(" ")
    a3 = IO.read(:line) |> String.trim |> String.split(" ")

    yoko = [ a1, a2, a3]
    cross = [
      [Enum.at(a1, 0), Enum.at(a2, 1), Enum.at(a3, 2)],
      [Enum.at(a1, 2), Enum.at(a2, 1), Enum.at(a3, 0)],
    ]
    tate = yoko |> Enum.zip |> Enum.map(&Tuple.to_list/1)

    lines = yoko ++ cross ++ tate

    n = IO.read(:line) |> String.trim |> String.to_integer

    bs =
      for _ <- 1..n do
        IO.read(:line) |> String.trim()
      end

    solve(lines, bs) |> IO.puts
  end

  def solve([], _) do
    "No"
  end

  def solve([head|tail], bs) do
    case (head -- bs) |> Enum.count == 0 do
      true -> "Yes"
      false -> solve(tail, bs)
    end
  end
end
