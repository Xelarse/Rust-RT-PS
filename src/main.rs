extern crate image;
use std::path::Path;

mod XelMath;

use XelMath::vector::Vec3;

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
            let hdr_col = Vec3::new(
                x as f64 / (OUTPUT_WIDTH - 1) as f64, 
                y as f64 / (OUTPUT_HEIGHT - 1) as f64, 
                0.25);

            let pixel = image_buffer.get_pixel_mut(x, OUTPUT_HEIGHT - (y + 1));

            let rgb_col = Vec3::hdr_to_rgb(hdr_col);
            pixel[0] = rgb_col.x() as u8;
            pixel[1] = rgb_col.y() as u8;
            pixel[2] = rgb_col.z() as u8;
            pixel[3] = 255;         
        }
    }

    if let Err(e) = image_buffer.save_with_format(&Path::new("outimage.png"), image::ImageFormat::Png){
        println!("Failed to write image with given error: {}", e);
    }

    let temp = Vec3::new(255.0, 240.0, 255.0);
    let temp2 = Vec3::new(200.0, 200.0, 200.0);
    let temp3 = temp - temp2;
    let temp4 = temp3 / 5.0;
    println!("{:?} - {:?} = {:?}; {:?} / 5 = {:?}", temp, temp2, temp3, temp3, temp4);
}