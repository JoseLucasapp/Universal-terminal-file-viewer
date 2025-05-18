use clap::{Arg, Command};
use image::GenericImageView;
use crossterm::style::{Stylize, PrintStyledContent};
use std::io::{stdout};
use crossterm::{execute, terminal, cursor};

fn main() {
    let matches = Command::new("see_file")
        .about("See a file on the terminal")
        .arg(
            Arg::new("image")
            .long("image")
            .short("i")
            .takes_value(true)
            .help("image path"),
        )
        .get_matches();

    if let Some(image_path) = matches.get_one::<String>("image"){
        
    }else{
        eprintln!("Image path not found: see_file -image <image_path>");
    }

    let img = match image::open("teste.jpg"){
        Ok(img) => img,
        Err(e)=>{
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    };
    let (w, h) = img.dimensions();
    let new_height = (h as f32 * 0.5) as u32;

    let resized = img.resize(80, new_height, image::imageops::FilterType::Nearest);

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();

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
