open Core

type cmd = Cd of string | Ls
type line = Command of cmd | Dir of string | File of int

let as_output_line line =
  match String.split ~on:' ' line with
  | [ "$"; "cd"; dir ] -> Command (Cd dir)
  | [ "$"; "ls" ] -> Command Ls
  | [ "dir"; dir ] -> Dir dir
  | [ size; _ ] -> File (Int.of_string size)
  | _ -> failwith "Bad command output line"
