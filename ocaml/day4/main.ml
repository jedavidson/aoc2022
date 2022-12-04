open Core

let hd_and_suc xs =
  match xs with x :: y :: _ -> (x, y) | _ -> failwith "Not enough elements"

let as_pair s =
  let as_assignment s' =
    hd_and_suc (String.split s' ~on:'-' |> List.map ~f:Int.of_string)
  in
  hd_and_suc @@ List.map ~f:as_assignment @@ String.split s ~on:','

let is_subsumed ((s, e), (s', e')) = (s >= s' && e <= e') || (s <= s' && e >= e')
let is_overlapping ((s, e), (s', e')) = Int.max s s' <= Int.min e e'

let assignments =
  In_channel.read_lines "../inputs/day4.txt" |> List.map ~f:as_pair

let () =
  let count ~f = Fn.compose List.length (List.filter ~f) in
  printf "Task 1: %d\n" @@ count assignments ~f:is_subsumed;
  printf "Task 1: %d\n" @@ count assignments ~f:is_overlapping
