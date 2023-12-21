use image::{ImageBuffer,GenericImageView,Pixel};
use std::{fs::File};
use std::fs;
use std::io::Write;
fn create_csv_string() -> String {
    let mut csv_string = String::from("Labels,");
    for i in 0..784 {
        csv_string+=&(format!("Pixel {}",i));
        if i != 783 {
            csv_string+=",";
        }
    }
    csv_string+="\n";
    csv_string
}
fn get_image_paths(dir_path:String)-> Vec<String> {
    let mut entries = fs::read_dir(dir_path).unwrap().map(|res| res.unwrap().path().into_os_string().into_string().unwrap()).collect();
    entries
}
fn main() {
    let root_data_training_path = r"C:\Users\MartinNammat\Documents\Programming-2\all_tests\rust_image_to_csv\archive\data\training_data".to_owned(); 
    let root_data_testing_path = r"C:\Users\MartinNammat\Documents\Programming-2\all_tests\rust_image_to_csv\archive\data\testing_data".to_owned();
    let image_path = r"C:\Users\MartinNammat\Documents\Programming-2\all_tests\rust_image_to_csv\out.jpg";

    let height = 28;
    let width= 28;


    let mut csv_string = create_csv_string(); 
    for num in 0..10 {
        let num_string = num.to_string();
        let entries = get_image_paths(root_data_testing_path.clone()+r"\"+&num_string); 
        println!("{}",&num_string);
        for path in entries {
            csv_string+=&num_string;
            csv_string+=",";
            let img = image::open(path).unwrap(); 
            let output = image::imageops::resize(&img,width,height,image::imageops::Triangle);

            for y in 0..output.height() {
                for x in 0..output.width() {
                    let pixel = output.get_pixel(x,y);
                    let rgb = pixel.to_rgb();
                    let [r,g,b] = rgb.0;
                    let f_r = r as f32;
                    let f_g = g as f32;
                    let f_b = b as f32;
                    let grey_val = (((f_r+f_g+f_b)/3.0)/128.0)-1.0;
                    csv_string+=&(grey_val.to_string());
                    csv_string+=",";
                }
            }
            csv_string+="\n";
        }
        
    }
    for num in 0..26 {

        let num_string = ((num+65) as u8) as char; 
        let entries = get_image_paths(root_data_testing_path.clone()+r"\"+&(num_string.to_string())); 
        println!("{}",&num_string);
        for path in entries {
            csv_string+=&((num+10).to_string());
            csv_string+=",";
            let img = image::open(path).unwrap(); 
            let output = image::imageops::resize(&img,width,height,image::imageops::Triangle);

            for y in 0..output.height() {
                for x in 0..output.width() {
                    let pixel = output.get_pixel(x,y);
                    let rgb = pixel.to_rgb();
                    let [r,g,b] = rgb.0;
                    let f_r = r as f32;
                    let f_g = g as f32;
                    let f_b = b as f32;
                    let grey_val = (((f_r+f_g+f_b)/3.0)/128.0)-1.0;
                    csv_string+=&(grey_val.to_string());
                    csv_string+=",";
                }
            }
            csv_string+="\n";
        }
    }


    csv_string+="\n";

    let mut file = File::create("num_c_test.csv").unwrap();
    file.write_all(csv_string.as_bytes()).unwrap();
    //image::save_buffer("image.png", &output, width, height, image::ColorType::Rgba8);
}
