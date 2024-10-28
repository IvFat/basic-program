use std::{thread::sleep, time::Duration, io::{self, Write}};
use crossterm::{
    cursor::{MoveTo, MoveRight},
    event::{read, Event, KeyCode},
    style::{Print},
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        Clear,
        ClearType::{All, CurrentLine},
        size
    },
    execute,
};

fn main() -> io::Result<()> {


    enable_raw_mode()?;

    // Get size of current terminal
    let (mut cols, mut rows) = size()?;

    let mut stdout = io::stdout();


    // Preparing terminal
    execute!(stdout,
        Clear(All),
        MoveTo(0, rows - 2)
    )?;


    // Loop for building interface
    let mut count: u16 = 0;
    loop {
        execute!(stdout,
            Print("_"),
            MoveRight(1)
        )?;

        count += 1;
        if count >= (cols / 2) + 1 {
            break;
        }
    }
        execute!(stdout,
            MoveTo(0, rows - 3),
            Print("Press H to help."),
            MoveTo(0, rows)
        )?;



    // Loop for handling events
    loop {
        match read()? {

            // Handling key events
            Event::Key(event) => match event.code {

                KeyCode::Char('q') | KeyCode::Char('Q') => {

                    execute!(stdout,
                        MoveTo(0, rows - 3),
                        Clear(CurrentLine),
                        Print("Are you sure you want to exit? (y/n)"),
                        MoveTo(0, rows)
                    )?;

                    match read()? {

                        Event::Key(event) => match event.code {    
                            KeyCode::Char('y') | KeyCode::Char('Y') => break,

                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                execute!(stdout,
                                    MoveTo(0, rows - 3),
                                    Clear(CurrentLine),
                                    Print("Cancel..."),
                                    MoveTo(0, rows)
                                )?;
    
                                sleep(Duration::new(1,0));
                                
                                execute!(stdout,
                                    MoveTo(0, rows - 3),
                                    Clear(CurrentLine),
                                    Print("Press H to help."),
                                    MoveTo(0, rows)
                                )?;
                            },
                            _ => {}
                        }
                        _ => {}
                    };
                },

                KeyCode::Char('g') | KeyCode::Char('G') => {

                    execute!(stdout,
                        MoveTo(0, rows - 3),
                        Clear(CurrentLine),
                        Print("Enter the website URL"),
                        MoveTo(0, rows)
                    )?;

                    match read()? {

                        Event::Key(event) => {

                            match event.code {
                                KeyCode::Char('*') => {
                                    execute!(stdout,
                                        Print(event.code),
                                        MoveRight(0)
                                    )?;
                                },

                                KeyCode::Enter => {
                                    execute!(stdout,
                                        MoveTo(0, rows - 4),
                                        Clear(CurrentLine),
                                    )?;

                                    print!("Going to {0}...", event.code);
                                    stdout.flush()?;

                                    execute!(stdout,
                                        MoveTo(0, rows)
                                    )?;
                                },
                                _ => {}
                            };
                        },
                        _ => {}
                    };
                },
                KeyCode::Char('c') | KeyCode::Char('C') => {

                    // Preparing the terminal
                    execute!(stdout,
                        Clear(All),
                        MoveTo(0, rows - 2)
                    )?;

                    // Loop for building interface
                    count = 0;
                    loop {
                        execute!(stdout,
                            Print("_"),
                            MoveRight(1)
                        )?;
                        count += 1;
                        if count >= (cols / 2) + 1 {
                            break;
                        }
                    }

                    execute!(stdout,
                        MoveTo(0, rows - 3),
                        Print("Press H to help."),
                        MoveTo(0, rows)
                        )?;

                }
                KeyCode::Char('h') | KeyCode::Char('H') => {
                    execute!(stdout,
                        MoveTo(0,0),
                        Print("Welcome to Simple Web-Browser. There are few commands you can use:"),
                        MoveTo(0, 1),
                        Print("\'H\' - Show this help"),
                        MoveTo(0, 2),
                        Print("\'G\' - Go to webpage"),
                        MoveTo(0, 3),
                        Print("\'C\' - Clear the screen"),
                        MoveTo(0, 4),
                        Print("\'Q\' - Exit SWB"),
                        MoveTo(0, rows)
                    )?;
                },
                _ => {}
            },
            _ => {}
        };
    }

    // Cleanup
    execute!(stdout,
        Clear(All),
        MoveTo(0, 0)
    )?;
    disable_raw_mode()?;
    Ok(())
}
