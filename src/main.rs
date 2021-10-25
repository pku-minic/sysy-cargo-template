use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Result, Write};

thread_local! {
  static GENERATOR: HashMap<char, &'static str> = {
    let mut map = HashMap::new();
    map.insert('e', r#"f_main [0]
  return 0
end f_main
"#);
    map.insert('t', r#"f_main [0]
  return 0
end f_main
"#);
    map.insert('r', r#"  .text
  .align 2
  .global main
main:
  li a0, 0
  ret
"#);
    map
  };
}

fn main() -> Result<()> {
  let mut args = env::args();
  args.next();
  GENERATOR.with(|g| {
    match (
      args.next(),
      args.next(),
      args.next(),
      args.next(),
      args.next(),
    ) {
      (Some(_), Some(m), Some(_), Some(_), Some(f)) if m == "-e" => {
        write!(File::create(f)?, "{}", g[&'e'])
      }
      (Some(_), Some(m), Some(_), Some(_), Some(f)) if m == "-t" => {
        write!(File::create(f)?, "{}", g[&'t'])
      }
      (Some(_), Some(_), Some(_), Some(f), None) => {
        write!(File::create(f)?, "{}", g[&'r'])
      }
      _ => panic!("invalid command line argument"),
    }
  })
}
