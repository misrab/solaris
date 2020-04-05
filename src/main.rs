
use std::process::{ Command, Output };
use std::str;
use std::io;

extern crate rand;
use rand::distributions::{Bernoulli, Distribution};

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

fn sample_char() -> String {
  let d = Bernoulli::new(0.5).unwrap();
  match d.sample(&mut rand::thread_rng()) {
    true => "-".to_string(),
    false => "|".to_string(),
    _ => "-".to_string(),
  }
}

fn random_row(cols: u32) -> String {
  (0..cols).map(|_| sample_char()).collect()
}

fn main() {
  let cols = get_terminal_cols().unwrap();
  
  for _ in 0..5{ 
      println!("{}", random_row(cols)); 
  }
}
