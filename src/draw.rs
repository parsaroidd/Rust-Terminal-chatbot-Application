use crossterm::{
    terminal::{self, Clear, ClearType}, 
    cursor::MoveTo,
    execute, 
    QueueableCommand
};
use std::io::Write;

const MARGIN: (u16, u16) = (7, 3);
/* The whole thing draws a box, how ever the you please, and the only thing you need to change
 * is this const :)))*/ 


pub fn draw_input_box(stream: &mut std::io::Stdout) -> (u16, u16)
{
    execute!(stream, Clear(ClearType::All));
    let (mut width, mut height) = terminal::size().unwrap_or((80, 20));
    let (corner1, corner2, corner3) = (height - MARGIN.0, width - 1, height - MARGIN.1);

    for row in 0..=height
    {
        if row == corner1 || row == corner3
        {
            for cell in 1..=corner2
            {
                stream.queue(MoveTo(cell, row));

                       if row == corner1 && cell == corner2 {
                    stream.write("┓".as_bytes());
                } else if row == corner3 && cell == corner2 { 
                    stream.write("┛".as_bytes());
                } else if row == corner1 && cell == 1       {
                    stream.write("┏".as_bytes());
                } else if row == corner3 && cell == 1       {
                    stream.write("┗".as_bytes());
                } else { 
                    stream.write("━".as_bytes());
                } 

            }
        }
        else if corner1 < row && row < corner3
        {
            stream.queue(MoveTo(1, row));
            stream.write("&".as_bytes());
            stream.queue(MoveTo(corner2, row));
            stream.write("&".as_bytes());
        }

    }

    return (width, height); 
}
