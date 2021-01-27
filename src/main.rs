extern crate image;
use std::path::Path;

mod XelMath;

use XelMath::vector;

fn main() {
    //Start by writing out a test image
    //Variables related to image size and image data
    const OUTPUT_WIDTH:u32 = 500;
    const OUTPUT_HEIGHT:u32 = 500;
    let mut image_buffer: image::RgbaImage = image::ImageBuffer::new(OUTPUT_WIDTH, OUTPUT_HEIGHT);
    
    // let mut pixel = image_buffer.get_pixel_mut(1, 1);
    // pixel[0] = 255;
    // pixel[1] = 200;
    // pixel[2] = 100;
    // pixel[3] = 255;

    //Start writing for the test image
    for y in 0..OUTPUT_HEIGHT
    {
        println!("Lines Remaining: {}", OUTPUT_HEIGHT - y);
        for x in 0..OUTPUT_WIDTH
        {            
            let r = x as f64 / (OUTPUT_WIDTH - 1) as f64;
            let g = y as f64 / (OUTPUT_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel = image_buffer.get_pixel_mut(x, OUTPUT_HEIGHT - (y + 1));
            pixel[0] = (255.999 * r) as u8;
            pixel[1] = (255.999 * g) as u8;
            pixel[2] = (255.999 * b) as u8;
            pixel[3] = 255;            
        }
    }

    if let Err(e) = image_buffer.save_with_format(&Path::new("outimage.png"), image::ImageFormat::Png){
        println!("Failed to write image with given error: {}", e);
    }

    let temp = vector::Vec3::new(255, 240, 255);
    println!("test {} {} {}", temp.x(), temp.y(), temp.z());
}