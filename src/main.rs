mod buffer;
mod draw;
pub use buffer::*;
pub use draw::*;
use std::io::stdout;

fn main()
{
    let mut stream = stdout();
    loop
    {
         let s = draw_input_control(&mut stream, 8, 3); 
         let (input, signal): (String, Signal) = buffer(&mut stream, s); 
         if signal == buffer::Signal::Exit
         {
             break;
         }
         println!("{input}");
    }
}
