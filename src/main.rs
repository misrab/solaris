extern crate solaris;

fn main() {
  println!("hello");
  solaris::primo::print_random_pattern();

  let mut ui = solaris::gui::UI::new();
  let mut app = solaris::gui::App::new();
  app.add_message(&mut ui, "hullo");
}
