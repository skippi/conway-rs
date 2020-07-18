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
    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let mut rng = rand::thread_rng();
    let points: Vec<(i32, i32)> = (0..81)
        .flat_map(|row| (0..41).map(move |col| (row, col)))
        .filter(|_| rng.gen())
        .collect();
    let mut conway = Conway::with_cells(&points);
    loop {
        conway = conway.next();
        queue!(stdout, crossterm::style::ResetColor, cursor::Hide)?;
        for i in 0..81 {
            for j in 0..41 {
                let symbol = match conway.get((i, j)) {
                    Cell::Alive => Color::White,
                    _ => Color::Black,
                };
                queue!(
                    io::stdout(),
                    cursor::MoveTo(i as u16, j as u16),
                    crossterm::style::SetBackgroundColor(symbol),
                    Print(" ")
                )?;
            }
        }
        stdout.flush()?;
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(_) => break,
                _ => (),
            }
        }
    }
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
