extern crate image;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};

const MESSAGE: &str = "This is the message.";
const KEY: &str = "This is the key.";
const INPUT_PATH: &str = "test16.png";
const OUTPUT_PATH: &str = "output.png";

fn main() {

    let encrypted_img = hide_msg_in_img(MESSAGE, KEY, INPUT_PATH).unwrap();
    encrypted_img.save(OUTPUT_PATH).unwrap();

}

fn hide_msg_in_img(msg: &str, _key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {

    let mut img = ImageReader::open(input_path)?.decode()?;
    let msg_bytes = msg.as_bytes();

    let mut msg_bits = Vec::new();
    for byte in msg_bytes {
        for i in 0..8 {
            msg_bits.push((byte & (1 << i)) / (1 << i));
        }
    }

    let mut p = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let mut pix = img.get_pixel(i, j);
            pix[p % 3] >>= 1;
            pix[p % 3] = (pix[p % 3] << 1) | msg_bits[p % msg_bits.len()];

            img.put_pixel(i, j, pix);

            p += 1;
        }
    }

    Ok(img)
}
