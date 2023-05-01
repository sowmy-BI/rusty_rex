use std::io::Write;

use crate::{cursor, queue, style, terminal, Queue, Result, Stdout, Stylize, Duration, sleep};

use crossterm::{event::{poll, read, KeyCode}, ExecutableCommand, };


pub fn draw_rex(stdout: &mut Stdout, x: u16, y: u16) -> Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x + 2, y - 1),
        style::Print("█▀".grey()),
        cursor::MoveTo(x, y),
        style::Print("⎴▛▜".grey())
    )?;
    stdout.flush()?;
    return Ok(());
}

fn user_input() -> Result<bool> {
    if poll(Duration::from_millis(50))? {
        match read()? {
            crossterm::event::Event::Key(key_event) => {
                if key_event.code == KeyCode::Up {
                    return Ok(true);
                }
                return Ok(false);
            }
            _ => return Ok(false),
        }
    }
    return Ok(false);
}


pub fn update_position(stdout: &mut Stdout, x: u16, y: &mut u16, height: u16) -> Result<()> {
    if user_input()? {
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorUp))?;
        *y -= 5;
        draw_rex(stdout, x, *y)?;
    }
    if *y < height { 
        *y += 1;
        draw_rex(stdout, x, *y)?;
        sleep(Duration::new(0, 9000000));
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorUp))?;
    }
    draw_rex(stdout, x, *y)?;
    return Ok(());
}




