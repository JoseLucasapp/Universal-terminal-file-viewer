use calamine::{open_workbook_auto, Reader, DataType};
use std::error::Error;

pub fn main(path: &str){
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
