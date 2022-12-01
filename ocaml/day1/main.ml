open Core

let rec parse_calories cs =
  match cs with
  | [] -> [ 0 ]
  | "" :: cs' -> 0 :: parse_calories cs'
  | c :: cs' ->
      let xs = parse_calories cs' in
      (List.hd_exn xs + int_of_string c) :: List.tl_exn xs

let read_calories =
  In_channel.read_lines "../inputs/day1.txt"
  |> parse_calories
  |> List.sort ~compare:Int.compare
  |> List.rev

let () =
  (* \n inserted because dune is ass *)
  Printf.printf "\n";
  let input = read_calories in
  Printf.printf "Task 1: %d\n" (List.hd_exn input);
  Printf.printf "Task 2: %d\n"
    (List.take input 3 |> List.fold ~init:0 ~f:Int.( + ))
