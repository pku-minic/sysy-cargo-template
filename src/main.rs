use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Result, Write};
use lazy_static::lazy_static;

lazy_static! {
  static ref GENERATOR: HashMap<char, &'static str> = {
    let mut map = HashMap::new();
    map.insert('e', r#"f_main [0]
  return 0
end f_main
"#);
    map.insert('t', r#"f_main [0]
  a0 = 0
  return
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
  match (
    args.next(),
    args.next(),
    args.next(),
    args.next(),
    args.next(),
  ) {
    (Some(_), Some(m), Some(_), Some(_), Some(f)) if m == "-e" => {
      write!(File::create(f)?, "{}", GENERATOR[&'e'])
    }
    (Some(_), Some(m), Some(_), Some(_), Some(f)) if m == "-t" => {
      write!(File::create(f)?, "{}", GENERATOR[&'t'])
    }
    (Some(_), Some(_), Some(_), Some(f), None) => {
      write!(File::create(f)?, "{}", GENERATOR[&'r'])
    }
    _ => panic!("invalid command line argument"),
  }
}
