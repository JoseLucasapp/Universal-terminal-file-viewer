use std::fs;

pub fn main(path: &str){
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