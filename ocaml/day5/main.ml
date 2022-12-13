open Core

let ( << ) = Fn.compose

let unparsed_crates, unparsed_moves =
  let rec split crates = function
    | line :: rest when String.is_prefix ~prefix:" 1" line ->
        (List.rev crates, List.tl_exn rest)
    | line :: rest -> split (line :: crates) rest
    | _ -> failwith "Bad input format"
  in
  split [] @@ In_channel.read_lines "../inputs/day5.txt"

let crates =
  let base_len =
    Option.value_exn
    @@ List.max_elt ~compare:Int.compare
    @@ List.map ~f:String.length unparsed_crates
  in
  let pad line =
    line ^ String.init (base_len - String.length line) ~f:(fun _ -> ' ')
  in
  let as_stack line =
    match List.hd_exn line with
    | crate when Char.is_alpha crate ->
        Some (List.drop_while ~f:(Char.( = ) ' ') @@ List.rev line)
    | _ -> None
  in
  List.filter_map ~f:as_stack
  @@ List.transpose_exn
  @@ List.rev_map ~f:(String.to_list << pad) unparsed_crates

let moves =
  let as_move line =
    match String.split ~on:' ' line with
    | [ "move"; amt; "from"; src; "to"; dst ] ->
        (Int.of_string amt, Int.of_string src - 1, Int.of_string dst - 1)
    | _ -> failwith "Bad input move"
  in
  List.map ~f:as_move unparsed_moves

let rearrange_crates ?(reorder = Fn.id) crates moves =
  let crates' = Array.of_list crates in
  let do_move (amt, src, dst) =
    let to_move, rest = List.split_n crates'.(src) amt in
    crates'.(dst) <- List.append (reorder to_move) crates'.(dst);
    crates'.(src) <- rest
  in
  List.iter ~f:do_move moves;
  crates'

let top_crate_message =
  String.of_char_list << Array.to_list << Array.map ~f:List.hd_exn

let () =
  printf "Task 1: %s\n" @@ top_crate_message
  @@ rearrange_crates crates moves ~reorder:List.rev;
  printf "Task 2: %s\n" @@ top_crate_message @@ rearrange_crates crates moves
