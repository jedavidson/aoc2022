open Core

let rec parse_calories cs =
  match cs with
  | [] -> [ 0 ]
  | "" :: cs' -> 0 :: parse_calories cs'
  | c :: cs' ->
      let xs = parse_calories cs' in
      (List.hd_exn xs + int_of_string c) :: List.tl_exn xs

let read_calories =
  In_channel.read_lines "./../inputs/day1.txt"
  |> parse_calories
  |> List.sort ~compare:Int.compare
  |> List.rev

let most_calories_carried = List.hd_exn

let calories_carried_by_top_3_elves total_cals =
  List.take total_cals 3 |> List.fold ~init:0 ~f:Int.( + )

let () =
  (* \n inserted because dune is ass *)
  Printf.printf "\nTask 1: %d\n" (most_calories_carried read_calories);
  Printf.printf "Task 2: %d\n" (calories_carried_by_top_3_elves read_calories)
