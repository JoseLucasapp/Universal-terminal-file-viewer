use image::GenericImageView;

fn main(){
    let img = image::open("teste.jpg").unwrap().grayscale();
    let (width, height) = img.dimensions();

    let chars = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];

    for y in (0..height).step_by(6){
        for x in (0..width).step_by(3){
            let pixel = img.get_pixel(x,y);
            let detax = pixel[0] as f32;
            let index = (detax / 255.0 * (chars.len() - 1) as f32).round() as usize;
            print!("{}", chars[index]);
        }
        println!();
    }
}