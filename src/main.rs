mod obstacle;
mod queue;

use std::{ io::{stdout, Stdout, Write}, time::Duration, thread::sleep};
use crossterm::{style, queue, terminal, cursor, Result, execute, ExecutableCommand};
use crossterm::style::Stylize;

use obstacle::Obstacle;
use rand::Rng;
use rand::{rngs::ThreadRng, thread_rng};

use queue::Queue;

fn main() -> Result<()> {
    let mut obs_q:Queue<Obstacle> = Queue::new();
    let mut rng: ThreadRng = thread_rng();
    let mut stdout = stdout();

    let (width, height) = terminal::size()?;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if obs_q.is_empty() || rng.gen_bool(0.1) {
            Obstacle::add(width-width/4, height/2, &mut obs_q, &mut rng);
        }
        
        Obstacle::draw(&mut stdout, &mut obs_q)?;
        
        sleep(Duration::new(0, 200000000));
                
        Obstacle::update(&mut obs_q);

        let obstacle = obs_q.get(0).unwrap();
        if obstacle.x < 10{
            obs_q.dequeue();
        }

    }
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    return Ok(());    
}
