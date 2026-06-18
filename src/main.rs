mod buffer;
pub use buffer::*;
use std::io::stdout;

fn main()
{
    let mut stream = stdout();
    let input: String = buffer(&mut stream);
    println!("{:?}", input);
}
