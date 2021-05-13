use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Paragraph;
use tui::Terminal;

struct Point {
    x: u8,
    y: u8,
}

struct Cursor {
    x: u8,
    y: u8,
}

impl Cursor {
    fn move_with(&mut self, dx: i8, dy: i8) {
        let x = (self.x as i8) + dx;
        let y = (self.y as i8) + dy;
        if x >= 0 && x < 3 && y >= 0 && y < 3 {
            self.x = x as u8;
            self.y = y as u8;
        }
    }
}

#[derive(Copy, Clone)]
enum Piece {
    O,
    X,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::O => write!(f, "O"),
            Piece::X => write!(f, "X"),
        }
    }
}

struct Board {
    pieces: [Option<Piece>; 9],
}

impl Board {
    fn new() -> Board {
        Board { pieces: [None; 9] }
    }

    fn index_from_point(point: &Point) -> usize {
        (point.x + point.y * 3) as usize
    }

    fn piece(&self, point: &Point) -> Option<Piece> {
        self.pieces[Board::index_from_point(point)]
    }

    fn put(&mut self, point: &Point, piece: Piece) {
        self.pieces[Board::index_from_point(point)] = Some(piece)
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut board = Board::new();
    let mut cursor = Cursor { x: 0, y: 0 };
    let mut next_turn = Piece::X;

    loop {
        terminal.clear()?;

        terminal.draw(|f| {
            for x in 0..3 {
                for y in 0..3 {
                    let piece = board.piece(&Point { x, y });

                    let cell = {
                        let cell = Paragraph::new(match piece {
                            None => "   ".to_string(),
                            Some(piece) => format!(" {} ", piece),
                        });

                        if x == cursor.x && y == cursor.y {
                            cell.style(Style::default().fg(Color::Black).bg(Color::White))
                        } else {
                            cell.style(Style::default().fg(Color::White).bg(Color::Black))
                        }
                    };

                    f.render_widget(
                        cell,
                        Rect::new((4 * x + 1) as u16, (2 * y + 1) as u16, 3, 1),
                    );

                    let text = Paragraph::new(format!("Next player: {}", next_turn));
                    f.render_widget(text, Rect::new(16, 1, 16, 1));

                    let text = Paragraph::new("Press q to exit.");
                    f.render_widget(text, Rect::new(16, 5, 16, 1));
                }
            }
        })?;

        if let Some(event) = io::stdin().keys().into_iter().next() {
            match event? {
                Key::Char('q') => break,

                Key::Left => cursor.move_with(-1, 0),
                Key::Right => cursor.move_with(1, 0),
                Key::Up => cursor.move_with(0, -1),
                Key::Down => cursor.move_with(0, 1),

                Key::Char(' ') => {
                    let point = Point {
                        x: cursor.x,
                        y: cursor.y,
                    };
                    if let None = board.piece(&point) {
                        board.put(&point, next_turn);

                        match next_turn {
                            Piece::X => next_turn = Piece::O,
                            Piece::O => next_turn = Piece::X,
                        }
                    }
                }

                _ => {}
            };
        }
    }

    Ok(())
}
