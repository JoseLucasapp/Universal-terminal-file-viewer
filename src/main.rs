use clap::{Arg, Command};

mod show_pdf;
use show_pdf :: main as show_pdf;

mod show_image;
use show_image :: main as show_image;

mod show_text;
use show_text :: main as show_text_file;

mod show_csv;
use show_csv :: main as show_csv;

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
        ).arg(
            Arg::new("download")
                .long("download")
                .short('d')
                .help("Download the image to the current directory")
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

    let download = matches.get_flag("download");

    if let Some(pdf_path) = matches.get_one::<String>("pdf") {
        show_pdf(pdf_path);
    }else if let Some(image_path) = matches.get_one::<String>("image") {
        show_image(image_path, width_scale, height_scale, download);
    }else if let Some(text_path) = matches.get_one::<String>("text") {
        show_text_file(text_path);
    }else if let Some(csv_path) = matches.get_one::<String>("csv") {
        show_csv(csv_path);
    }else{
        eprintln!("Please specify either --image or --pdf");
    }

}