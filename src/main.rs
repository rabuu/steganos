extern crate image;

use image::{DynamicImage, GenericImageView, ImageError, Pixel, io::Reader as ImageReader};

const MESSAGE: &str = "This is the message.";
const KEY: &str = "This is the key.";
const INPUT_PATH: &str = "test16.png";
const OUTPUT_PATH: &str = "output.png";

fn main() {

    let encrypted_img = hide_msg_in_img(MESSAGE, KEY, INPUT_PATH).unwrap();
    encrypted_img.save(OUTPUT_PATH).unwrap();

}

fn hide_msg_in_img(message: &str, key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {
    let mut img = ImageReader::open(input_path)?.decode()?;

    let mut p = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let pix = img.get_pixel(i, j);
            println!("{}: {}", p, pix[0]);
            
            p += 1;
        }
    }

    Ok(img)
}

// fn image2matrix(img_path: &str) -> Result<ImgMatrix, ImageError> {
//     let img = ImageReader::open(img_path)?.decode()?;
//     let mut matrix: ImgMatrix = Vec::new();

//     for i in 0..img.width() {
//         matrix.push(Vec::new());
//         for j in 0..img.height() {
//             matrix[i as usize].push(img.get_pixel(i as u32, j as u32).to_rgb());
//         } 
//     }

//     Ok(matrix)
// }

// fn encrypt(img: ImgMatrix, msg: String) -> ImgMatrix {
//     fn hide_bit() -> Rgb<u8> {
//         Rgb([255, 255, 255])
//     }
//     let mut encr: ImgMatrix = Vec::new();
//     for i in 0..img.len() {
//         encr.push(Vec::new());
//         for j in 0..img[i].len() {
//             encr[i].push(hide_bit());
//         }
//     }
//     encr
// }
