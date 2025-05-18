use image::GenericImageView;
use crossterm::style::{Stylize, PrintStyledContent};
use std::io::{stdout};
use crossterm::{execute, terminal};

fn main() {
    let img = match image::open("o.jpg"){
        Ok(img) => img,
        Err(e)=>{
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    };
    let resized = img.resize(80, 40, image::imageops::FilterType::Nearest);

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    for y in 0..resized.height(){
        for x in 0..resized.width(){
            let pixel = resized.get_pixel(x,y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let style = "█".with(crossterm::style::Color::Rgb {r,g,b});
            execute!(stdout, PrintStyledContent(style)).unwrap();
        }

        println!();
    }
}
