val _ =
  case [3] of
    [] => 1
  | [_] => 2
  | [_, 7] => 3
  | [_, 5] => 4
  | [7, 7] => 5
  | [_, _] => 6
  | x :: _ => x
