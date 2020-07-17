mod conway;

use crate::conway::Conway;
use crossterm::event::Event;
use crossterm::style::{Color, Print};
use crossterm::{cursor, event, terminal};
use crossterm::{execute, queue};
use rand::Rng;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let mut conway = Conway::new();
    let mut rng = rand::thread_rng();
    for i in 0..81 {
        for j in 0..41 {
            conway.set_alive(conway::Point(i, j), rng.gen());
        }
    }
    loop {
        conway = conway.cycle();
        queue!(
            stdout,
            crossterm::style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide
        )?;
        for i in 0..81 {
            for j in 0..41 {
                let symbol = if conway.is_alive(conway::Point(i, j)) {
                    Color::White
                } else {
                    Color::Black
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
        if event::poll(Duration::from_secs(1))? {
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
