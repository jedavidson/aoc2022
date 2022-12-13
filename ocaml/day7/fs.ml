open Core

type node = Dir of string * int * node list

let init name = Dir (name, 0, [])
let empty = init "/"
let size (Dir (_, size, _)) = size

let rec touch file_size cwd (Dir (name, size, children) as dir) =
  match cwd with
  | basename :: rest when String.( = ) basename name ->
      let children' = List.map ~f:(touch file_size rest) children in
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

let rec fold init (Dir (_, _, children) as dir) ~f =
  let init' = List.fold ~init ~f:(fold ~f) children in
  f init' dir
