open Core
module CharSet = Set.Make (Char)

let get_priority item =
  match item with
  | 'a' .. 'z' -> 1 + Char.to_int item - Char.to_int 'a'
  | 'A' .. 'Z' -> 27 + Char.to_int item - Char.to_int 'A'
  | _ -> failwith "Bad item type"

let as_char_set = Fn.compose CharSet.of_list String.to_list

let common_item rucksacks =
  CharSet.choose_exn @@ List.reduce_exn rucksacks ~f:CharSet.inter

let get_common_in_compartments rs =
  let l = String.length rs in
  common_item @@ List.map ~f:as_char_set
  @@ [ String.slice rs 0 (l / 2); String.slice rs (l / 2) l ]

let get_badge group = common_item @@ List.map group ~f:as_char_set

let sum_common_item_priorities rucksacks =
  List.map rucksacks ~f:get_common_in_compartments
  |> List.fold ~init:0 ~f:(fun prio item -> prio + get_priority item)

let sum_badge_priorities rucksacks =
  List.chunks_of rucksacks ~length:3
  |> List.map ~f:get_badge
  |> List.fold ~init:0 ~f:(fun prio item -> prio + get_priority item)

let rucksacks = In_channel.read_lines "../inputs/day3.txt"

let () =
  printf "\nTask 1: %d\n" @@ sum_common_item_priorities rucksacks;
  printf "Task 2: %d\n" @@ sum_badge_priorities rucksacks
