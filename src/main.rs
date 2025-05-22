use clap::{Arg, Command};
use image::GenericImageView;
use crossterm::style::{Stylize, PrintStyledContent};
use std::io::{stdout};
use crossterm::{execute, terminal, cursor};
use std::fs;
use std::error::Error;
use calamine::{open_workbook_auto, Reader, DataType};

mod show_pdf;
use show_pdf :: main as show_pdf;

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
        .arg(
            Arg::new("text")
                .long("text")
                .short('t')
                .value_name("TEXT_PATH")
                .help("text path to render in terminal")
                .required(false),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .short('c')
                .value_name("CSV_PATH")
                .help("csv path to render in terminal")
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
    }else if let Some(text_path) = matches.get_one::<String>("text") {
        show_text_file(text_path);
    }else if let Some(csv_path) = matches.get_one::<String>("csv") {
        show_csv(csv_path);
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
    let (_w, h) = img.dimensions();
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

fn show_text_file(path: &str){
    match fs::read_to_string(path){
        Ok(content) =>{
            println!("--- File content ---\n");
            println!("{}", content);
        }
        Err(e)=>{
            eprintln!("Failed to open file: '{}' : {}",path, e);
        }
    }
}

fn show_csv(path: &str){
    match read_csv(path){
        Ok(_)=>{},
        Err(e)=>{
            eprintln!("Failed to open file: '{}' : {}",path, e);
        }
    }
}

fn read_csv(path: &str) -> Result<(), Box<dyn Error>>{
    let mut workbook  = open_workbook_auto(path)?;
    let sheet_names = workbook.sheet_names().to_owned();

    if sheet_names.is_empty(){
        return Err("No sheets found".into());
    }

    let range = workbook.worksheet_range(&sheet_names[0]).ok_or("Sheet not found")??;

    println!("--- Excel: {} ---\n", sheet_names[0]);
   
    for row in range.rows(){
        for cell in row{
            let content = match cell{
                DataType::String(s) => s.to_string(),
                DataType::Float(f) => format!("{:.2}", f),
                DataType::Int(i) => i.to_string(),
                DataType::Bool(b) => b.to_string(),
                DataType::Empty => "".to_string(),
                _ => "[?]".to_string(),
            };
            println!("{:<20}", content);
        }
        println!();
    }
    Ok(())
}
