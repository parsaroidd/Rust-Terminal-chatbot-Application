# Elahe

## WHAT IS GOING TO BE
<ul>
  <li>Probably temple os themed,</li>
  <li>A smart Elahe that extends its knowledge outside of the runtime</li>
  <li>And random stuff that I don't really know YET</li>
</ul>
 

## SETUP
Just set up a cargo directory and add crossterm to it.
<br>
then you can just put all the things in my /src to your /src and hit a `cargo run` or whatever.
<br>
OR you can specify binaries in your `Cargo.toml`, the [[bin]] parts, like me and run 
<br>
`>> cargo run --bin source` 
<br> <br>
For now they don't have any diffrence but this application probably is going to have multiple binaries in future so yeah.

<img src="https://github.com/parsaroidd/Rust-Terminal-Elahe-Application/blob/main/img/Screenshot_2026-06-19_19-09-35.jpg">
<img src="https://github.com/parsaroidd/Rust-Terminal-Elahe-Application/blob/main/img/image.png">
## 20th of June 2026, 

I commited in a bach and didn't write the updates in the commmit, So I'll write it here! 
for draw.rs, I just re aplied the logic to draw the main screen, in buffer I add a Signal Enum that actually tell main function to exit, and main is now a loop that renders a frame every two seconds.

I just relized how buffer.rs is a mess, I mean the Key and Mouse match statements are readable, but not the code that is written on top of that, I must handle in the future, If I see that it needs to be.

