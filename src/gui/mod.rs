use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{stdout, Write},
    //sync::mpsc,
    //thread,
    //time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

/*enum Event<I> {*/
    //Input(I),
    //Tick,
/*}*/


fn new_crossterm() -> Result<tui::terminal::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    
    // "?" handles error conversion, need to ? and convert it otherwise
    // answer.map_err(|e| Box::new(e) as Box<dyn error::Error>)
    Ok(Terminal::new(backend)?)
}

pub fn draw_ui() -> Result<(), Box<dyn Error>> {
  let mut terminal = new_crossterm()?;
  
  terminal.draw(|mut f| {
    let size = f.size();
    Block::default()
        .title("Block")
        .borders(Borders::ALL)
        .render(&mut f, size);
  });

  Ok(())
}

#[test]
fn test_run() -> Result<(), Box<dyn Error>> {
  println!("run");


  Ok(())
}
