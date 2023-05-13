extern crate nom;
extern crate asalang;

use asalang::{program, start_interpreter, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"1+true"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      let result = start_interpreter(&tree);
      println!("{:?}", result);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }

    
  Ok(())
}
