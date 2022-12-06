open Core

let datastream =
  In_channel.read_lines "../inputs/day6.txt" |> List.hd_exn |> String.to_list

let first_marker_pos data marker_size =
  let is_marker marker =
    List.length marker = marker_size
    && not (List.contains_dup ~compare:compare_char marker)
  in
  let rec iter_markers xs pos =
    match xs with
    | c :: c' :: cs' ->
        if List.length cs' >= marker_size - 2 then
          if is_marker (c :: c' :: List.take cs' (marker_size - 2)) then
            pos + marker_size
          else iter_markers (c' :: cs') (pos + 1)
        else -1
    | _ -> -1
  in
  iter_markers data 0

let () =
  printf "Task 1: %d\n" @@ first_marker_pos datastream 4;
  printf "Task 2: %d\n" @@ first_marker_pos datastream 14
