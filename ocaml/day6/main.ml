open Core

let datastream =
  In_channel.read_lines "../inputs/day6.txt" |> List.hd_exn |> String.to_list

let is_marker = Fn.compose not (List.contains_dup ~compare:compare_char)

let first_marker_pos data marker_size =
  let rec iter_markers stream pos =
    let marker = List.take stream marker_size in
    if List.length marker = marker_size then
      if is_marker marker then pos + marker_size
      else iter_markers (List.tl_exn stream) (pos + 1)
    else -1
  in
  iter_markers data 0

let () =
  printf "Task 1: %d\n" @@ first_marker_pos datastream 4;
  printf "Task 2: %d\n" @@ first_marker_pos datastream 14
