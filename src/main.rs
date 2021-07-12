extern crate image;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};

type BitVec = Vec<u8>;

const MESSAGE: &str = "This is the message.";
const KEY: &str = "This is the key.";
const INPUT_PATH: &str = "test2560.png";
const OUTPUT_PATH: &str = "output.png";

fn main() {

    let encrypted_img = hide_msg_in_img(MESSAGE, KEY, INPUT_PATH).unwrap();
    encrypted_img.save(OUTPUT_PATH).unwrap();

}

fn hide_msg_in_img(msg: &str, key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {

    let mut img = ImageReader::open(input_path)?.decode()?;
    let msg_bits: BitVec = str_to_bitvec(msg);
    let key_bits: BitVec = str_to_bitvec(key);

    let mut pos = 0;
    let mut msg_pos = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let mut pix = img.get_pixel(i, j);

            match key_bits[pos % key_bits.len()] {
                0 => {
                    pix[pos % 3] >>= 1;
                    pix[pos % 3] = (pix[pos % 3] << 1) | msg_bits[msg_pos % msg_bits.len()];
                    msg_pos += 1;
                },
                1 => {
                    pix[pos % 3] >>= 2;
                    pix[pos % 3] = ((pix[pos % 3] << 2) | msg_bits[msg_pos % msg_bits.len()] << 1) | msg_bits[(msg_pos + 1) % msg_bits.len()];
                    msg_pos += 2;
                },
                _ => panic!("Unexpected! BitVec contains something else than 0 or 1")
            }

            img.put_pixel(i, j, pix);
            pos += 1;

        }
    }
    Ok(img)
}

fn str_to_bitvec(string: &str) -> BitVec {
    let mut bitvec: BitVec = Vec::new();
    for byte in string.as_bytes() {
        for i in 0..8 {
            bitvec.push((byte & (1 << i)) / (1 << i));
        }
    }
    bitvec
}
