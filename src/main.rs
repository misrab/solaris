use std::process::{ Command, Output };
use std::str;
use std::io;

fn get_terminal_cols() -> Result<u32, io::Error> {
  match Command::new("tput").arg("cols").output() {
    Ok(Output{ stdout, .. }) => {
      //let cols = stdout.parse::<u32>().unwrap();
      let stdout_str = str::from_utf8(&stdout).unwrap().replace("\n", "");
      let stdout_int = stdout_str.parse::<u32>().unwrap(); 
      Ok(stdout_int)
    },
    Err(err) => {
      Err(err)
    }
  }
}


fn random_row(cols: u32) -> String {
  (0..cols).map(|_| "m").collect()
}

fn main() {
  let cols = get_terminal_cols().unwrap();
  
  println!("{}", random_row(cols)); 
}
