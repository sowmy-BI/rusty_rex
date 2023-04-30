use crossterm::ExecutableCommand;
use rand::Rng;

use crate::{cursor, queue, style, terminal, Queue, Result, Stdout, Stylize, ThreadRng};

pub struct Obstacle {
    pub x: u16,
    pub y: u16,
    pub size: u16,
}

impl Obstacle {
    pub fn draw(stdout: &mut Stdout, obs_q: &mut Queue<Obstacle>) -> Result<()> {
        for index in 0..obs_q.length() {
            let obstacle = obs_q.get(index).unwrap();
            queue!(
                stdout,
                cursor::MoveTo(obstacle.x, obstacle.y),
                style::Print("┗┃┛".grey())
            )?;
            stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        }
        return Ok(());
    }

    pub fn add(start_x: u16, start_y: u16, obs_q: &mut Queue<Obstacle>, rng: &mut ThreadRng) {
        let new_size = rng.gen_range(1..3);

        obs_q.enqueue(Obstacle {
            x: start_x,
            y: start_y,
            size: new_size,
        })
    }

    pub fn update(obs_q: &mut Queue<Obstacle>) {
        for index in 0..obs_q.length() {
            let obstacle = obs_q.get(index).unwrap();
            obs_q.update(
                index,
                Obstacle {
                    x: obstacle.x - 1,
                    y: obstacle.y,
                    size: obstacle.size,
                },
            );
        }
    }
}
