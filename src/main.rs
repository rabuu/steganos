extern crate image;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, Pixel, Rgb, io::Reader as ImageReader};

type ImgMatrix = Vec<Vec<Rgb<u8>>>;

fn main() {

    let x: ImgMatrix = image2matrix("test16.png").unwrap();
    let y = encrypt(x, "hey".to_string());
    save_img(y, "testsave.png")

}

fn image2matrix(img_path: &str) -> Result<ImgMatrix, ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    let mut matrix: ImgMatrix = Vec::new();

    for i in 0..img.width() {
        matrix.push(Vec::new());
        for j in 0..img.height() {
            matrix[i as usize].push(img.get_pixel(i as u32, j as u32).to_rgb());
        } 
    }

    Ok(matrix)
}

fn encrypt(img: ImgMatrix, msg: String) -> ImgMatrix {
    fn hide_bit() -> Rgb<u8> {
        Rgb([255, 255, 255])
    }
    let mut encr: ImgMatrix = Vec::new();
    for i in 0..img.len() {
        encr.push(Vec::new());
        for j in 0..img[i].len() {
            encr[i].push(hide_bit());
        }
    }
    encr
}

fn save_img(imgmatrix: ImgMatrix, path: &str) {
    let mut img: DynamicImage = DynamicImage::new_rgb8(imgmatrix.len() as u32, imgmatrix[0].len() as u32);
    for i in 0..imgmatrix.len() {
        for j in 0..imgmatrix[i].len() {
            img.put_pixel(i as u32, j as u32, imgmatrix[i][j].to_rgba());
        }
    }
    img.save(path).unwrap();
}

    // let mut x: u8 = 0b1101_1111;
    // let bit: bool = true;
    // if x & 1 != 0 && !bit {
    //     x -= 1;
    // } else if x & 1 == 0 && bit { 
    //     x += 1;
    // }
    // println!("{}", x);
