use std::io::{self, Write};
use crossterm::{
    cursor::{MoveTo},
    event::{read, Event, KeyCode, KeyEvent},
    style::{Print},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType::{All}, size},
    execute,
};

fn main() -> io::Result<()> {

    enable_raw_mode();
    
    // Get size of current terminal
    let (mut cols, mut rows) = size()?;

    let mut stdout = io::stdout();

    // Preparing terminal
    execute!(
        stdout,
        Clear(All),
        MoveTo(1, 0)
    )?;
    print!("Press \'q\' to exit.");
    stdout.flush()?;

    // Loop for handling events
    loop {
        match read()? {

            // Handling 'q' pressing event
            Event::Key(event) => if event.code == KeyCode::Char('q') {
                                    break;       
                                 }

            // Handling resize and print current size of terminal
            Event::Resize(mut cols, mut rows) => {
                (cols, rows) = size()?;
                
                execute!(
                    stdout,
                    MoveTo((cols / 2) - 15, rows / 2),
                    Clear(All)
                )?;

                print!("Current terminal size: {cols}x{rows}");
                stdout.flush()?;
                

                execute!(
                    stdout,
                    MoveTo(1, 0),
                )?;

                print!("Press \'q\' to exit.");
                stdout.flush()?;
            },

            _ => {} // Ignore other cases

        }
    }

    // Cleanup
    execute!(
        stdout,
        Clear(All),
        MoveTo(0, 0)
    )?;
    disable_raw_mode();
    Ok(())
}
