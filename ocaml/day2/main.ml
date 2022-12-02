open Core

let input_path = "../inputs/day2.txt"

let as_int c =
  match c with
  | 'A' | 'B' | 'C' -> Char.to_int c - Char.to_int 'A'
  | 'X' | 'Y' | 'Z' -> Char.to_int c - Char.to_int 'X'
  | _ -> failwith "Bad line entry"

let to_round s =
  match String.split s ~on:' ' with
  | them :: us :: _ -> (as_int @@ String.get them 0, as_int @@ String.get us 0)
  | _ -> failwith "Bad line format"

(* With each shape mapped into Z_3, we can determine scores formulaically *)
let score (them, us) = 1 + us + (3 * ((us - them + 1) % 3))
let score_with_strategy (them, us) = (3 * us) + (1 + ((us + them - 1) % 3))
let total_score ~f rounds = List.map ~f rounds |> List.fold ~init:0 ~f:( + )

let () =
  let rounds = In_channel.read_lines input_path |> List.map ~f:to_round in
  printf "Task 1: %d\n" @@ total_score rounds ~f:score;
  printf "Task 2: %d\n" @@ total_score rounds ~f:score_with_strategy
