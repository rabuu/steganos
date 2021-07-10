extern crate image;

use image::{GenericImageView, ImageError, Pixel, Rgb, io::Reader as ImageReader};

type ImgMatrix = Vec<Vec<Rgb<u8>>>;

fn main() {

    let x: ImgMatrix = image2matrix("test16.png").unwrap();
    println!("{:?}", x);

}

fn image2matrix(img_path: &str) -> Result<ImgMatrix, ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    let mut matrix: Vec<Vec<Rgb<u8>>> = Vec::new();

    for i in 0..img.width() {
        matrix.push(Vec::new());
        for j in 0..img.height() {
            matrix[i as usize].push(img.get_pixel(i as u32, j as u32).to_rgb());
        } 
    }

    Ok(matrix)
}


    // let mut x: u8 = 0b1101_1111;
    // let bit: bool = true;
    // if x & 1 != 0 && !bit {
    //     x -= 1;
    // } else if x & 1 == 0 && bit { 
    //     x += 1;
    // }
    // println!("{}", x);
