open Core
module CharSet = Set.Make (Char)

let get_priority item =
  match item with
  | 'a' .. 'z' -> 1 + Char.to_int item - Char.to_int 'a'
  | 'A' .. 'Z' -> 27 + Char.to_int item - Char.to_int 'A'
  | _ -> failwith "Bad item type"

let as_char_set r =
  List.fold (String.to_list r) ~init:CharSet.empty ~f:CharSet.add

let get_common_in_compartments rs =
  let l = String.length rs in
  let fst_comp, snd_comp =
    ( as_char_set @@ String.slice rs 0 (l / 2),
      as_char_set @@ String.slice rs (l / 2) l )
  in
  CharSet.choose_exn @@ CharSet.inter fst_comp snd_comp

let get_badge group =
  CharSet.choose_exn
  @@
  match List.map group ~f:as_char_set with
  | fst :: snd :: thd :: _ -> CharSet.inter (CharSet.inter fst snd) thd
  | _ -> failwith "Not enough elves in group"

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
