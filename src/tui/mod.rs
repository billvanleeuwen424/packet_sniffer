pub mod ui;

use std::io::{self, Stdout};

use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::{CompletedFrame, Frame, Terminal};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn enter() -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?.execute(Hide)?;
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn exit(&mut self) -> io::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?.execute(Show)?;
        Ok(())
    }

    pub fn draw<F: FnOnce(&mut Frame)>(&mut self, f: F) -> io::Result<CompletedFrame<'_>> {
        self.terminal.draw(f)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}
