use pdf_extract;

pub fn main(path: &str){
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