extern crate image;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};

type BitVec = Vec<u8>;

const MESSAGE: &str = "This is the message.";
const KEY: &str = "This is the key.";
const INPUT_PATH: &str = "test16.png";
const OUTPUT_PATH: &str = "output.png";
const ENCRYPTED_INPUT_PATH: &str = "test16-encrypted.png";

fn main() {

    // let encrypted_img = hide_msg_in_img(MESSAGE, KEY, INPUT_PATH).unwrap();
    // encrypted_img.save(OUTPUT_PATH).unwrap();

    let decrypted_msg = extract_msg_from_img(ENCRYPTED_INPUT_PATH, KEY);

}

fn hide_msg_in_img(msg: &str, key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {

    let mut img = ImageReader::open(input_path)?.decode()?;
    let msg_bits: BitVec = str_to_bitvec(msg);
    let key_bits: BitVec = str_to_bitvec(key);

    let mut key_pos = 0;
    let mut msg_pos = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let mut pix = img.get_pixel(i, j);

            for k in 0..3 {
                match key_bits[key_pos % key_bits.len()] {
                    0 => {
                        pix[k] >>= 1;
                        pix[k] = (pix[k] << 1) | msg_bits[msg_pos % msg_bits.len()];
                        msg_pos += 1;
                    },
                    1 => {
                        pix[k] >>= 2;
                        pix[k] = ((pix[k] << 2) | msg_bits[msg_pos % msg_bits.len()] << 1) | msg_bits[(msg_pos + 1) % msg_bits.len()];
                        msg_pos += 2;
                    },
                    _ => panic!("Unexpected! BitVec contains something else than 0 or 1")
                }
                img.put_pixel(i, j, pix);
                key_pos += 1;
            }
        }
    }
    Ok(img)
}

fn extract_msg_from_img(input_path: &str, key: &str) -> Result<String, ImageError> {

    let img = ImageReader::open(input_path)?.decode()?;
    let key_bits = str_to_bitvec(key);
    let mut msg_bytes: Vec<u8> = Vec::new();

    let mut key_pos = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let pix = img.get_pixel(i, j);

            for k in 0..3 {
                match key_bits[key_pos % key_bits.len()] {
                    0 => {
                        print!("{}", pix[k] & 1);
                    },
                    1 => {
                        print!("{}", (pix[k] & (1 << 1)) / 2);
                        print!("{}", pix[k] & 1);
                    },
                    _ => panic!()
                }
                key_pos += 1;
            }
        }
    }
    Ok("dummy".to_string())
}

fn str_to_bitvec(string: &str) -> BitVec {
    let mut bitvec: BitVec = Vec::new();
    for byte in string.as_bytes() {
        for i in 0..8 {
            bitvec.push((byte & (1 << (7 - i))) / (1 << (7 - i)));
        }
    }
    bitvec
}
