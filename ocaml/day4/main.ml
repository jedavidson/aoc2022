open Core

let as_pairs s =
  match
    List.map ~f:Int.of_string @@ String.split_on_chars s ~on:[ ','; '-' ]
  with
  | [ s; e; s'; e' ] -> ((s, e), (s', e'))
  | _ -> failwith "Bad line format"

let is_subsumed ((s, e), (s', e')) = (s >= s' && e <= e') || (s <= s' && e >= e')
let is_overlapping ((s, e), (s', e')) = Int.max s s' <= Int.min e e'

let () =
  let pairs =
    In_channel.read_lines "../inputs/day4.txt" |> List.map ~f:as_pairs
  in
  printf "Task 1: %d\n" @@ List.count pairs ~f:is_subsumed;
  printf "Task 1: %d\n" @@ List.count pairs ~f:is_overlapping
