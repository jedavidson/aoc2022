open Core
open Types

let input = In_channel.read_lines "../inputs/day2.txt" |> List.map ~f:to_round
let total_score rounds = List.map ~f:score rounds |> List.fold ~init:0 ~f:( + )

let total_score_with_strategy rounds =
  List.map ~f:score_with_strategy rounds |> List.fold ~init:0 ~f:( + )

let () =
  printf "Task 1: %d\n" @@ total_score input;
  printf "Task 2: %d\n" @@ total_score_with_strategy input
