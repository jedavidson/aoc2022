open Core

type opponent = A | B | C
type action = X | Y | Z

let as_opponent s =
  match s with "A" -> A | "B" -> B | "C" -> C | _ -> failwith "Bad opponent"

let as_action s =
  match s with "X" -> X | "Y" -> Y | "Z" -> Z | _ -> failwith "Bad action"

let to_round s =
  match String.split s ~on:' ' with
  | them :: you :: _ -> (as_opponent them, as_action you)
  | _ -> failwith "Bad line format"

let score round =
  match round with
  | A, X -> 3 + 1
  | A, Y -> 6 + 2
  | A, Z -> 0 + 3
  | B, X -> 0 + 1
  | B, Y -> 3 + 2
  | B, Z -> 6 + 3
  | C, X -> 6 + 1
  | C, Y -> 0 + 2
  | C, Z -> 3 + 3

let score_with_strategy round =
  match round with
  | A, X -> 0 + 3
  | A, Y -> 3 + 1
  | A, Z -> 6 + 2
  | B, X -> 0 + 1
  | B, Y -> 3 + 2
  | B, Z -> 6 + 3
  | C, X -> 0 + 2
  | C, Y -> 3 + 3
  | C, Z -> 6 + 1
