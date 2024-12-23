let rec read_lines () =
  let line = try Some (read_line ()) with End_of_file -> None in
  match line with
  | Some line -> Seq.cons (String.to_seq line) (read_lines ())
  | None -> Seq.empty

let rec print_board_rec (lines : char Seq.t Seq.t) : unit =
  let print_line (line : char Seq.t) : unit =
    let _ = print_string (String.of_seq line) in
    let _ = print_newline () in
    ()
  in

  match lines () with
  | Cons (line, rest) ->
      let _ = print_line line in
      print_board_rec rest
  | Nil -> ()

let print_board (lines : char Seq.t Seq.t) : unit =
  let _ = print_string "Lines: " in
  let _ = print_int (Seq.length lines) in
  let _ = print_newline () in

  print_board_rec lines

let flip (lines : char Seq.t Seq.t) : char Seq.t Seq.t =
  Seq.map (fun line -> line |> List.of_seq |> List.rev |> List.to_seq) lines

let transpose (lines : char Seq.t Seq.t) : char Seq.t Seq.t =
  Seq.transpose lines

let rec count_horizontal_1 (line : char Seq.t) : int =
  let matches = Seq.equal ( = ) (Seq.take 4 line) (String.to_seq "XMAS") in
  let here = if matches then 1 else 0 in
  let rest =
    match line () with
    | Seq.Nil -> 0
    | Seq.Cons (_, rest) -> count_horizontal_1 rest
  in
  here + rest

let count_horizontal (lines : char Seq.t Seq.t) : int =
  lines |> Seq.map count_horizontal_1 |> Seq.fold_left ( + ) 0

let count_diagonal (lines : char Seq.t Seq.t) : int =
  let diag_extractor acc list =
    let acc2 = Seq.cons Seq.empty acc in
    let list2 =
      Seq.append list
        (Seq.repeat ' ' |> Seq.take (Seq.length acc2 - Seq.length list))
    in
    Seq.map2 Seq.append acc2 (list2 |> Seq.map (fun e -> [ e ] |> List.to_seq))
  in
  let acc_start : char Seq.t Seq.t =
    Seq.repeat Seq.empty |> Seq.take (Seq.length lines)
  in
  let acc_final : char Seq.t Seq.t =
    Seq.fold_left diag_extractor acc_start lines
  in
  acc_final |> count_horizontal

let part1 (lines : char Seq.t Seq.t) : int =
  let boards_h =
    [
      (* L -> R *)
      lines;
      (* R -> L *)
      lines |> flip;
      (* U -> D *)
      lines |> transpose;
      (* D -> U *)
      lines |> transpose |> flip;
    ]
  in
  let boards_d =
    [
      (* UL -> DR *)
      lines;
      (* UR -> DL *)
      lines |> flip;
      (* DL -> UR *)
      lines |> transpose |> flip;
      (* DR -> UL *)
      lines |> flip |> transpose |> flip;
    ]
  in
  let result_h = boards_h |> List.map count_horizontal in
  let result_d = boards_d |> List.map count_diagonal in
  let count_h = result_h |> List.fold_left ( + ) 0 in
  let count_d = result_d |> List.fold_left ( + ) 0 in
  count_h + count_d

let select (skip : int) (keep : int) (s : 'a Seq.t) : 'a Seq.t =
  s |> Seq.drop skip |> Seq.take keep

let rec count_pattern_from (x : int) (y : int) (lines : char Seq.t Seq.t)
    (pattern : char Seq.t Seq.t) : int =
  let selection = lines |> select y 3 |> Seq.map (select x 3) in

  let height = selection |> Seq.length in
  let width = selection |> Seq.map Seq.length |> Seq.fold_left Int.max 0 in

  if height < 3 || width < 3 then 0
  else
    let here =
      if Seq.equal (Seq.equal (fun s p -> p = '.' || s = p)) selection pattern
      then 1
      else 0
    in
    let rest_x = count_pattern_from (x + 1) y lines pattern in
    let rest_y =
      if x = 0 then count_pattern_from 0 (y + 1) lines pattern else 0
    in

    here + rest_x + rest_y

let count_pattern (lines : char Seq.t Seq.t) (pattern : char Seq.t Seq.t) : int
    =
  count_pattern_from 0 0 lines pattern

let part2 (lines : char Seq.t Seq.t) : int =
  let patterns =
    [
      [ "M.S"; ".A."; "M.S" ];
      [ "M.M"; ".A."; "S.S" ];
      [ "S.M"; ".A."; "S.M" ];
      [ "S.S"; ".A."; "M.M" ];
    ]
  in
  let pattern_counts =
    patterns
    |> List.map (fun p -> p |> List.map String.to_seq |> List.to_seq)
    |> List.map (count_pattern lines)
  in
  pattern_counts |> List.fold_left ( + ) 0

let program () =
  let lines = read_lines () in
  let _ = print_endline (part1 lines |> Int.to_string) in
  let _ = print_endline (part2 lines |> Int.to_string) in
  ()
