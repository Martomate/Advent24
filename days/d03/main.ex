defmodule D3 do
  def run do
    input = read_input()
    pairs = parse_input(input)

    result1 = calc_part1(pairs)
    result2 = calc_part2(pairs)

    IO.puts(Integer.to_string(result1))
    IO.puts(Integer.to_string(result2))
  end

  defp read_input() do
    IO.read(:eof)
  end

  defp parse_input(input) do
    Regex.scan(~r/(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)/, input)
    |> Enum.map(fn [_ | m] ->
      case m do
        ["mul", a, b] ->
          {a, ""} = Integer.parse(a)
          {b, ""} = Integer.parse(b)
          {:mul, a, b}

        [_, _, _, "do"] ->
          {:do}

        [_, _, _, _, "don't"] ->
          {:dont}
      end
    end)
  end

  defp calc_part1(pairs) do
    pairs
    |> Enum.map(
      &case &1 do
        {:mul, a, b} -> a * b
        _ -> 0
      end
    )
    |> Enum.sum()
  end

  defp calc_part2(pairs) do
    {_, result2} =
      pairs
      |> Enum.reduce({true, 0}, fn e, {enabled, sum} ->
        case e do
          {:mul, a, b} ->
            add = if enabled, do: a * b, else: 0
            {enabled, sum + add}

          {:do} ->
            {true, sum}

          {:dont} ->
            {false, sum}
        end
      end)

    result2
  end
end
