use crossterm::{
    style::Stylize,
    event::{ Event, read, EnableMouseCapture}, 
    execute,
};
use std::io::stdout;

enum Color {
    Red, Green, Yellow, Blue, Magenta, White, Black, 
}

struct Stroke
{
    x    : u16, 
    y    : u16, 
    chr  : char, 
    color: Color, 
}


impl Stroke
{
    pub fn conf(&mut self)
    {
	let formatted = match self.color
	{
	    Color::Magenta		=> format!("{}", self.chr.magenta()),
	    Color::Blue 		=> format!("{}", self.chr.blue()),
	    Color::Green		=> format!("{}", self.chr.green()),
	    Color::Yellow		=> format!("{}", self.chr.yellow()), 
	    Color::Red			=> format!("{}", self.chr.red()),
	    Color::White		=> format!("{}", self.chr.blue()),
	    Color::Black		=> format!("{}", self.chr.black()),
	    _ => format!("{}", self.chr.green()), 
	};
	println!("{}", formatted); 
    }
}

fn scribble() //-> Signal
{
    loop
    {
	match read().unwrap()
	{
	    Event::Mouse(event) =>
	    {
		println!("{:?}", event); 
	    },
	    _ => {}, 
	} 
    }
}
fn main() 
{
    let mut stream = stdout();
    execute!(stream, EnableMouseCapture);
    scribble();
    let mut this = Stroke { 
	x: 10,
	y: 10,
      chr: 'x',
    color:  Color::Yellow, 
	};
    this.conf();
}
