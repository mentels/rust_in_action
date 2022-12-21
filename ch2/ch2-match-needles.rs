fn main() {
  let _needle = 42;                 // <1>
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for item in &haystack {
    let result = match item {
      1 ..= 41 => "lower range",
      42 | 132 => "hit",
      _ => "miss",
    };

      println!("{}: {}", item, result);
      // print_result(item, &result)
  }
}

// fn print_result(item :i32, result: String) -> None {
  // println!("{}: {}", item, result)
// }