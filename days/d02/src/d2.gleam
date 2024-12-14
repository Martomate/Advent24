import gleam/erlang
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

// Great article: https://www.benjaminpeinhardt.com/error-handling-in-gleam/

pub type Error {
  ReadLinesError(erlang.GetLineError)
  ParsingError
}

pub fn main() -> Result(_, Error) {
  use lines <- result.try(
    read_lines()
    |> result.map_error(ReadLinesError),
  )

  use rows <- result.try(
    parse_lines(lines)
    |> result.replace_error(ParsingError),
  )

  let #(safe, not_safe) = list.partition(rows, is_safe)

  let num_safe = list.length(safe)
  let num_almost_safe = list.count(not_safe, is_almost_safe)

  io.println(int.to_string(num_safe))
  io.println(int.to_string(num_safe + num_almost_safe))

  Ok(Nil)
}

pub fn is_safe(levels: List(Int)) -> Bool {
  let pairs = list.window(levels, 2)
  let diffs =
    list.map(pairs, fn(p) {
      case p {
        [a, b] -> b - a
        _ -> panic
      }
    })
  let asc = list.all(diffs, fn(d) { d >= 1 && d <= 3 })
  let desc = list.all(diffs, fn(d) { d <= -1 && d >= -3 })

  asc || desc
}

fn is_almost_safe(levels: List(Int)) -> Bool {
  list.range(0, list.length(levels) - 1)
  |> list.map(fn(i) {
    list.flatten([list.take(levels, i), list.drop(levels, i + 1)])
  })
  |> list.map(is_safe)
  |> list.any(fn(b) { b == True })
}

fn read_lines() -> Result(List(String), _) {
  erlang.get_line("")
  |> result.map(remove_trailing_newline)
  |> result.then(fn(line) {
    use rest <- result.try(read_lines())
    Ok(list.prepend(rest, line))
  })
  |> result.try_recover(fn(e) {
    case e {
      erlang.Eof -> Ok([])
      e -> Error(e)
    }
  })
}

fn remove_trailing_newline(s: String) -> String {
  case string.ends_with(s, "\n") {
    True -> string.drop_end(s, up_to: 1)
    False -> s
  }
}

fn parse_lines(lines: List(String)) -> Result(List(List(Int)), _) {
  lines
  |> list.map(fn(line) {
    line
    |> string.split(on: " ")
    |> list.map(int.parse)
    |> result.all
  })
  |> result.all
}
