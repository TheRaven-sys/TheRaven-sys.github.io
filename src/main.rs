use std::{thread, time};
use std::io::{self, Write, stdout};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, size, Clear, ClearType},
    cursor::{Hide, MoveTo, Show}
            
    };
mod frame;
fn main() -> io::Result<()>{
    let duration = time::Duration::from_secs(5);
    let (cols, rows) = size().unwrap();
    let (mut a, mut b) = (0.5,0.5);
    
    execute!(stdout(), EnterAlternateScreen)?;
    let mut frame = frame::renderPipeline::Frame::new((rows as usize).try_into().expect("rows failed to convert to usize"),
                                                         (cols as usize).try_into().expect("cols failed to convert to usize"),
                                                          a , b);
    frame.renderFrame(a, b);
    execute!(stdout(), MoveTo(0,0), Hide)?;
    for y in 0..rows {
        let start = y * cols;
        let end = start + cols;
        println!("{}", frame.buffer.iter().collect::<String>());
    }
    loop {
        a += 0.08;
        b += 0.04;
        frame.buffer.fill(' ');
        frame.renderFrame(a, b);
        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0))?;
        println!("{}", frame.buffer.iter().copied().collect::<String>());
        thread::sleep(time::Duration::from_millis(8));
    }

    execute!(stdout(),Clear(ClearType::All), LeaveAlternateScreen, Show)?;
    Ok(())
}