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
//use tui::backend::Backend;
use tui::Frame;
use tui::{backend::CrosstermBackend, Terminal};
use tui::style::{Color, Style};
use tui::widgets::{
    Widget, Block, Borders, Text, Paragraph,
};
use tui::layout::{
    Layout, Constraint, Direction,
};

fn new_crossterm() -> Result<tui::terminal::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    
    // "?" handles error conversion, need to ? and convert it otherwise
    // answer.map_err(|e| Box::new(e) as Box<dyn error::Error>)
    Ok(Terminal::new(backend)?)
}


fn draw(mut f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);

    let mut text = Vec::<Text>::new();
    for (i, s) in app.messages.iter().enumerate() {
      let mut s2 = s.clone();
      if i > 0 {
          s2.insert_str(0, "\n");
      }
      text.push(Text::raw(s2));
    }

    //let text = [ Text::raw("testing"), Text::styled("\nsecond", Style::default().fg(Color::Blue)), ];
    let mut paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
    
    paragraph.render(&mut f, size);
}

/*pub fn initialise_ui() -> Result<(), Box<dyn Error>> {*/
  //let mut terminal = new_crossterm()?;
 
  //terminal.clear()?;

  //let mut app = App::new();
  //app.add_message("hello".to_string());

  //terminal.draw(|mut f| draw(&mut f, &mut app));
  //Ok(())
/*}*/

pub struct UI {
  terminal: tui::terminal::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
}

impl UI {
  pub fn new() -> Self {
    let mut terminal = new_crossterm().unwrap();
    terminal.clear().unwrap();
  
    UI{
      terminal: terminal,
    }
  }
}

pub struct App {
  messages: Vec<String>,
}

impl App {
  pub fn new() -> Self {
    App{
      messages: Vec::new(),
    }
  }

  pub fn add_message(&mut self, ui: &mut UI, s: &str) {
    self.messages.push(s.to_string());
    ui.terminal.draw(|mut f| draw(&mut f, &self));
  }
}


#[test]
fn test_run() -> Result<(), Box<dyn Error>> {
  println!("run");


  Ok(())
}
