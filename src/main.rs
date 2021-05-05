use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    Ok(())
}
