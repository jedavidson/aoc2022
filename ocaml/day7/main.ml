open Core

let root =
  let output_lines =
    In_channel.read_lines "../inputs/day7.txt"
    |> List.tl_exn
    |> List.map ~f:Output.as_output_line
  in
  let rec mkfs cwd fs = function
    | Output.Command (Cd "..") :: rest -> mkfs (List.tl_exn cwd) fs rest
    | Output.Command (Cd dir) :: rest -> mkfs (dir :: cwd) fs rest
    | Output.Command Ls :: rest -> mkfs cwd fs rest
    | Output.Dir dir :: rest -> mkfs cwd (Fs.mkdir dir (List.rev cwd) fs) rest
    | Output.File size :: rest -> mkfs cwd (Fs.touch size (List.rev cwd) fs) rest
    | [] -> fs
  in
  mkfs [ "/" ] Fs.empty output_lines

let total_dir_size_under_100k =
  let f sum (Fs.Dir (_, size, _)) = if size <= 100_000 then sum + size else sum in
  Fs.fold root ~init:0 ~f

let size_of_smallest_viable_dir =
  let root_size = Fs.size root in
  let space = 70_000_000 - root_size in
  let f min (Fs.Dir (_, size, _)) =
    if space + size >= 30_000_000 then Int.min min size else min
  in
  Fs.fold root ~init:root_size ~f

let () =
  printf "Task 1: %d\n" @@ total_dir_size_under_100k;
  printf "Task 2: %d\n" @@ size_of_smallest_viable_dir
