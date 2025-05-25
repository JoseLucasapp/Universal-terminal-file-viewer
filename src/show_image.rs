use image::GenericImageView;
use std::io::{stdout};
use crossterm::style::{Stylize, PrintStyledContent};
use crossterm::{execute, terminal, cursor};

pub fn main(path: &str, width_scale: u32, height_scale: u32, download: bool) {
    let img = match image::open(path){
        Ok(img) => img,
        Err(e)=>{
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    };
    let (_w, h) = img.dimensions();
    let new_height = ((h as f32 * 0.5) / (10.0 / height_scale.max(1) as f32)) as u32;
    let resized = img.resize(80, new_height, image::imageops::FilterType::Nearest);

    if download {
        let output_path = "outputFromSeeFile_follow_joselucasapp_on_github.png";
        match resized.save(output_path) {
            Ok(_) => println!("Image saved to {}", output_path),
            Err(e) => eprintln!("Failed to save image: {}", e),
        }
    }

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    for y in 0..resized.height(){
        for x in 0..resized.width(){
            let pixel = resized.get_pixel(x,y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let block = "█".repeat(width_scale as usize);
            let style = block.with(crossterm::style::Color::Rgb {r,g,b});
            execute!(stdout, PrintStyledContent(style)).unwrap();
        }

        println!();
    }
}