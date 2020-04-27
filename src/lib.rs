//pub use self::primo::print_random_pattern;

#[cfg(test)]
mod tests {
    #[test]
  fn it_works() {
          assert_eq!(2 + 2, 4);
    }
}

// just getting the hang of Rust
pub mod primo;

// a terminal gui for chat
pub mod gui;

// p2p messaging experiment
pub mod messaging;

pub mod datarythms;
