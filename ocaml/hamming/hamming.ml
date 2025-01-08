type nucleotide = A | C | G | T


let same_length coll1 coll2 = List.length coll1 = List.length coll2

let tuple_elems_do_not_match (x, y) = x <> y
let (&) = Fun.compose
let curr_distance acc = ((+) acc) & Bool.to_int & tuple_elems_do_not_match
let just x = Ok x

let calc_distance = just & List.fold_left curr_distance 0 

let determine_error_msg coll1 coll2  = match (coll1, coll2) with
  | [], _ -> "left strand must not be empty"
  | _, [] -> "right strand must not be empty"
  | _, _ ->  "left and right strands must be of equal length"

let to_error msg = Error msg

let error_msg coll1 coll2 = determine_error_msg coll1 coll2 |> to_error 


let hamming_distance coll1 coll2 = if same_length coll1 coll2 then calc_distance (List.combine coll1 coll2) else error_msg coll1 coll2
