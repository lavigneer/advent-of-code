let file = "bin/day1.prod"

module IntCompare = struct
  type t = int

  let compare = Int.compare
end

module IntHeap = Adventofcode.Heap.Heap (IntCompare)
module IntMap = Map.Make (Int)

let heapOne, heapTwo =
  let ic = open_in file in
  let rec line_reader channel heapOne heapTwo =
    match input_line channel with
    | line ->
      let split_line = Str.split (Str.regexp " +") line in
      (match split_line with
       | first :: second :: _ ->
         let () = IntHeap.add heapOne (int_of_string first) in
         let () = IntHeap.add heapTwo (int_of_string second) in
         line_reader channel heapOne heapTwo
       | _ -> line_reader channel heapOne heapTwo)
    | exception End_of_file -> heapOne, heapTwo
  in
  line_reader ic (IntHeap.create ()) (IntHeap.create ())
;;

let rec compute_difference heapOne heapTwo diff =
  match IntHeap.pop_min heapOne, IntHeap.pop_min heapTwo with
  | Some first, Some second ->
    compute_difference heapOne heapTwo (Int.add diff (Int.abs (Int.sub first second)))
  | _ -> diff
;;

let () =
  let diff = compute_difference heapOne heapTwo 0 in
  Printf.printf "Part 1 - %d\n" diff
;;

let listOne, listTwoOccurrences =
  let ic = open_in file in
  let rec line_reader channel listOne listTwoOccurrences =
    match input_line channel with
    | line ->
      let split_line = Str.split (Str.regexp " +") line in
      (match split_line with
       | first :: second :: _ ->
         line_reader
           channel
           (int_of_string first :: listOne)
           (IntMap.update
              (int_of_string second)
              (fun v ->
                match v with
                | Some v -> Some (Int.add v 1)
                | None -> Some 1)
              listTwoOccurrences)
       | _ -> line_reader channel listOne listTwoOccurrences)
    | exception End_of_file -> listOne, listTwoOccurrences
  in
  line_reader ic [] IntMap.empty
;;

let () =
  let sum =
    List.fold_left
      (fun res v ->
        res
        +
        match IntMap.find_opt v listTwoOccurrences with
        | Some a -> v * a
        | None -> 0)
      0
      listOne
  in
  Printf.printf "Part 2 - %d\n" sum
;;
