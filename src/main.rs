use clap::{Arg, Command};
use image::GenericImageView;
use crossterm::style::{Stylize, PrintStyledContent};
use std::io::{stdout};
use crossterm::{execute, terminal, cursor};
use pdf_extract;

fn main() {
    let matches = Command::new("see_file")
        .about("See a file on the terminal")
        .arg(
            Arg::new("image")
                .long("image")
                .short('i')
                .value_name("IMAGE_PATH")
                .help("image path to render in terminal")
                .required(false),
        )
        .arg(
            Arg::new("width")
                .long("width")
                .short('w')
                .value_name("1-2")
                .help("Width scale factor (1-2)")
                .required(false),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .short('v')
                .value_name("1-9")
                .help("Height scale factor (1-9)")
                .required(false),
        )
        .arg(
            Arg::new("pdf")
                .long("pdf")
                .short('p')
                .value_name("PDF_PATH")
                .help("pdf path to render in terminal")
                .required(false),
        )
        .get_matches();


    let width_scale = matches
        .get_one::<String>("width")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|v| (1..=2).contains(v))
        .unwrap_or(1);

    let height_scale = matches
        .get_one::<String>("height")
        .and_then(|s| s.parse::<u32>().ok())
        .filter(|v| (1..=9).contains(v))
        .unwrap_or(1);

    if let Some(pdf_path) = matches.get_one::<String>("pdf") {
        show_pdf(pdf_path);
    }else if let Some(image_path) = matches.get_one::<String>("image") {
        show_image(image_path, width_scale, height_scale);
    }else{
        eprintln!("Please specify either --image or --pdf");
    }

}


fn show_image(path: &str, width_scale: u32, height_scale: u32){
    let img = match image::open(path){
        Ok(img) => img,
        Err(e)=>{
            eprintln!("Failed to open image: {}", e);
            std::process::exit(1);
        }
    };
    let (w, h) = img.dimensions();
    let new_height = ((h as f32 * 0.5) / (10.0 / height_scale.max(1) as f32)) as u32;
    let resized = img.resize(80, new_height, image::imageops::FilterType::Nearest);

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

fn show_pdf(path: &str){
    match pdf_extract::extract_text(path){
        Ok(text)=>{
            println!("--- PDF Content ---\n");

            let clean = text
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            let pretty = clean
            .replace(". ", ".\n\n")
            .replace("! ", "!\n")
            .replace("? ", "?\n");


            println!("{}", pretty)
        }
        Err(e)=>{
            eprintln!("Failed to open pdf: {}", e);
        }
    }
}