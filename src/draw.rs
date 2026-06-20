use crossterm::{
    terminal::{self, Clear, ClearType}, 
    cursor::MoveTo,
    execute, 
    QueueableCommand
};
use std::{
    io::Write, 
    thread, time
};

const DISTANCE_FROM_TOP: u16 = 1;

pub fn draw_input_control(stream: &mut std::io::Stdout, ending_row: u16, starting_row: u16) -> (u16, u16)
{
    execute!(stream, Clear(ClearType::Purge));
    thread::sleep(time::Duration::from_secs(2));
    execute!(stream, Clear(ClearType::All));

    let (mut width, mut height) = terminal::size().unwrap_or((80, 20));

    let (corner1, edge_width, corner3) = (height - ending_row, width - 1, height - starting_row);
    let (scrstr, scrend) = ( DISTANCE_FROM_TOP , height - (ending_row - starting_row + 5) );

    for row in 0..=height
    {
        if row == corner1 || row == corner3
        {
            for cell in 1..=edge_width
            {
                stream.queue(MoveTo(cell, row));

                       if row == corner1 && cell == edge_width {
                    stream.write("┓".as_bytes());
                } else if row == corner3 && cell == edge_width { 
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
        else if (corner1 < row && row < corner3) || (scrstr < row && scrend > row)
        {
            stream.queue(MoveTo(1, row));
            stream.write("&".as_bytes());
            stream.queue(MoveTo(edge_width, row));
            stream.write("&".as_bytes());
        }
        else if row == scrstr || row == scrend
        {
            for cell in 1..=edge_width
            {
                stream.queue(MoveTo(cell, row));

                       if row == scrstr && cell == edge_width {
                    stream.write("┓".as_bytes());
                } else if row == scrend && cell == edge_width { 
                    stream.write("┛".as_bytes());
                } else if row == scrstr && cell == 1       {   
                    stream.write("┏".as_bytes());
                } else if row == scrend && cell == 1       {   
                    stream.write("┗".as_bytes());
                } else { 
                    stream.write("━".as_bytes());
                } 
            }
        }

    }

    return (width, height); 
}
