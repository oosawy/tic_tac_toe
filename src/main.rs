use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Terminal;

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cursor = Rect::default();

    loop {
        terminal.clear()?;

        terminal.draw(|f| {
            let text = Paragraph::new("+");
            f.render_widget(text, Rect::new(cursor.x, cursor.y, 3, 1));
        })?;

        if let Some(event) = io::stdin().keys().into_iter().next() {
            match event? {
                Key::Char('q') => break,

                Key::Left => cursor.x -= 4,
                Key::Right => cursor.x += 4,
                Key::Up => cursor.y -= 2,
                Key::Down => cursor.y += 2,

                _ => {}
            };
        }
    }

    Ok(())
}
