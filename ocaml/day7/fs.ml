open Core

type node = Dir of string * int * node list

let init name = Dir (name, 0, [])
let empty = init "/"
let size (Dir (_, size, _)) = size

let rec touch file_size cwd (Dir (name, size, children) as dir) =
  match cwd with
  | basename :: cwd' when String.( = ) basename name ->
      let children' = List.map ~f:(touch file_size cwd') children in
      Dir (name, size + file_size, children')
  | _ -> dir

let rec mkdir new_dir cwd (Dir (name, size, children) as dir) =
  if String.( = ) name @@ List.hd_exn cwd then
    let children' =
      if List.length cwd = 1 then init new_dir :: children
      else List.map ~f:(mkdir new_dir (List.tl_exn cwd)) children
    in
    Dir (name, size, children')
  else dir

let fold fs ~init ~f =
  let rec fold' acc (Dir (_, _, children) as dir) =
    f (List.fold ~init:acc ~f:fold' children) dir
  in
  fold' init fs
