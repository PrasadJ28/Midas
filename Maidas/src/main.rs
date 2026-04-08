use std::io;

use ratatui::{DefaultTerminal, Frame};
#[derive(Debug, Default)]
pub struct App {
    counter:u8,
    exit: bool,
}
fn main() -> io::Result<()>{
    ratatui::run(|terminal| App::default().run(terminal))
}

impl App{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
