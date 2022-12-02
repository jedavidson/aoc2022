open Core

let rec parse_calories cs =
  match cs with
  | [] -> [ 0 ]
  | "" :: rest -> 0 :: parse_calories rest
  | cals :: rest ->
      let rest' = parse_calories rest in
      (List.hd_exn rest' + Int.of_string cals) :: List.tl_exn rest'

let input =
  In_channel.read_lines "../inputs/day1.txt"
  |> parse_calories
  |> List.sort ~compare:(Fn.flip Int.compare)

let () =
  printf "Task 1: %d\n" (List.hd_exn input);
  printf "Task 2: %d\n" (List.take input 3 |> List.fold ~init:0 ~f:( + ))
