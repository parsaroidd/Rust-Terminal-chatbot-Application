//┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//┃                                                                                                                                   ┃
//┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

fn walter(stream: &mut std::io::Stdout ) -> (u16, u16)                       //this is walter, he draws boxes.
{

    let (mut width, mut height) = terminal::size().unwrap();
    let mut ceil = "━";
    let margin_bottom = 4;

        stream.queue(MoveTo(0, height - margin_bottom))                                 .unwrap()
    ;   stream.write("┏".as_bytes())                                                    .unwrap()
    ;   stream.write(ceil.repeat((width - 2).into()).as_bytes())                        .unwrap()
    ;   stream.write("┓".as_bytes())                                                    .unwrap()
    ;

        stream.queue(MoveTo(0, height - margin_bottom + 1))                             .unwrap()
    ;   stream.write("┃".as_bytes())                                                    .unwrap()
    ;   stream.queue(MoveTo(width, height - margin_bottom + 1))                         .unwrap()
    ;   stream.write("┃".as_bytes())                                                    .unwrap()
    ;
        stream.queue(MoveTo(0, height - margin_bottom + 2))                             .unwrap()
    ;   stream.write("┃".as_bytes())                                                    .unwrap()
    ;   stream.queue(MoveTo(width, height - margin_bottom + 2))                         .unwrap()
    ;   stream.write("┃".as_bytes())                                                    .unwrap()
    ;


        stream.queue(MoveTo(0, height - margin_bottom +3))                              .unwrap()
    ;   stream.write("┗".as_bytes())                                                    .unwrap()
    ;   stream.write(ceil.repeat((width - 2).into()).as_bytes())                        .unwrap()
    ;   stream.write("┛".as_bytes())                                                    .unwrap()
    ;
    stream.flush().unwrap();

    return (width, height)
}

fn main()
{
    let mut stream = stdout();
        execute!(stream, Clear(ClearType::All)).unwrap();
        thread::sleep(Duration::from_secs(1));
        execute!(stream, SetForegroundColor(Color::White), Clear(ClearType::All)).unwrap();
        let instance = walter(&mut stream);
    
}













use std::io::{Write, stdout};
use crossterm::terminal::{self, Clear, ClearType, ScrollUp};
use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor};
use crossterm::cursor::{MoveTo};
use crossterm::{execute, QueueableCommand};
use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers};
use std::time::Duration;
use std::thread;
