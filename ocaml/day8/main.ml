open Core

let ( << ) = Fn.compose
let ( --> ) = List.range
let ( <-- ) a b = List.rev (a --> b)
let max l ~compare = Option.value_exn @@ List.max_elt ~compare l

let trees =
  In_channel.read_lines "../inputs/day8.txt"
  |> Array.of_list_map ~f:(Array.map ~f:Char.get_digit_exn << String.to_array)

let h, w = (Array.length trees, Array.length trees.(0))

let is_visible r c =
  let f c' = trees.(r).(c') < trees.(r).(c) in
  let horizontally = List.for_all (0 --> c) ~f || List.for_all (c + 1 --> w) ~f in
  let f r' = trees.(r').(c) < trees.(r).(c) in
  let vertically = List.for_all (0 --> r) ~f || List.for_all (r + 1 --> h) ~f in
  horizontally || vertically

let scenic_score r c =
  let f c' = trees.(r).(c') >= trees.(r).(c) in
  let from_left = Option.value (List.find ~f (0 <-- c)) ~default:0 in
  let from_right = Option.value (List.find ~f (c + 1 --> w)) ~default:(w - 1) in
  let f r' = trees.(r').(c) >= trees.(r).(c) in
  let from_above = Option.value (List.find ~f (0 <-- r)) ~default:0 in
  let from_below = Option.value (List.find ~f (r + 1 --> h)) ~default:(h - 1) in
  (c - from_left) * (from_right - c) * (r - from_above) * (from_below - r)

let num_visible_trees =
  let f r = List.count (0 --> w) ~f:(is_visible r) in
  List.sum (module Int) (0 --> h) ~f

let max_scenic_score =
  let f r = max (List.map (0 --> w) ~f:(scenic_score r)) ~compare:Int.compare in
  max (List.map (0 --> h) ~f) ~compare:Int.compare

let () =
  printf "Task 1: %d\n" @@ num_visible_trees;
  printf "Task 2: %d\n" @@ max_scenic_score
