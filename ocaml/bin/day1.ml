let file = "bin/day1.prod"

let () =
  let ic = open_in file in
  let rec line_reader channel current_max running_total =
    match input_line channel with
    | "" -> line_reader channel (max current_max running_total) 0
    | line ->
        line_reader channel current_max (running_total + int_of_string line)
    | exception End_of_file -> max current_max running_total
  in
  Printf.printf "Part 1 - %d\n" (line_reader ic 0 0)

let () =
  let ic = open_in file in
  let rec line_reader channel totals running_total =
    match input_line channel with
    | "" -> line_reader channel (running_total :: totals) 0
    | line -> line_reader channel totals (running_total + int_of_string line)
    | exception End_of_file ->
        Seq.fold_left ( + ) 0
          (Seq.take 3
             (List.to_seq
                (List.sort
                   (fun x y -> ~-(compare x y))
                   (running_total :: totals))))
  in
  Printf.printf "Part 2 - %d\n" (line_reader ic [] 0)
