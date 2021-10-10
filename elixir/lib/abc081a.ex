require Integer

defmodule Main do
  def main do
    IO.read(:line)
      |> String.trim
      |> String.codepoints
      |> Enum.map(&String.to_integer/1)
      |> Enum.sum
      |> IO.puts
  end
end
