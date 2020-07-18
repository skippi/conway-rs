mod conway;

use crate::conway::{Cell, Conway};
use crossterm::event::Event;
use crossterm::style::{Color, Print};
use crossterm::{cursor, event, terminal};
use crossterm::{execute, queue};
use rand::Rng;
use std::io;
use std::io::Write;
use std::time::Duration;

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;
    let mut rng = rand::thread_rng();
    let (rows, cols) = terminal::size()?;
    let mut conway = Conway::with_iter(points(rows as i32, cols as i32).filter(|_| rng.gen()));
    loop {
        conway = conway.next();
        draw_conway(&mut stdout, &conway, rows as i32, cols as i32)?;
        stdout.flush()?;
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(_) => break,
                _ => (),
            }
        }
    }
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}

fn points(width: i32, height: i32) -> impl Iterator<Item = (i32, i32)> {
    (0..width).flat_map(move |row| (0..height).map(move |col| (row, col)))
}

fn draw_conway<W: Write>(
    writer: &mut W,
    conway: &Conway,
    width: i32,
    height: i32,
) -> crossterm::Result<()> {
    queue!(writer, crossterm::style::ResetColor, cursor::Hide)?;
    for point in points(width, height) {
        let (row, col) = point;
        let cell = conway.get(point);
        let background_color = pick_cell_color(&cell);
        queue!(
            writer,
            cursor::MoveTo(row as u16, col as u16),
            crossterm::style::SetBackgroundColor(background_color),
            Print(" ")
        )?
    }
    Ok(())
}

fn pick_cell_color(cell: &Cell) -> Color {
    match cell {
        Cell::Alive => Color::White,
        _ => Color::Black,
    }
}
