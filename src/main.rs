extern crate solaris;

fn main() {
  println!("hello");
  solaris::primo::print_random_pattern();

  let _ = solaris::gui::draw_ui();
}
