extern crate image;

use std::str::from_utf8;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};

type BitVec = Vec<u8>;

const MESSAGE: &str = "This is the message.*[END]*";
const KEY: &str = "This is the key.";
const INPUT_PATH: &str = "tests/test16.png";
const OUTPUT_PATH: &str = "tests/output.png";
const ENCRYPTED_INPUT_PATH: &str = "tests/test16-encrypted.png";

const EOM_IDENTIFIER: &str = "*[END]*";

fn main() {

    // let encrypted_img = hide_msg_in_img(MESSAGE, KEY, INPUT_PATH).unwrap();
    // encrypted_img.save(OUTPUT_PATH).unwrap();

    let decrypted_msg = extract_msg_from_img(ENCRYPTED_INPUT_PATH, KEY).unwrap();
    println!("{}", decrypted_msg);

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
                    _ => panic!("BitVec contains something else than 0 or 1")
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
    let mut msg_bits: BitVec = Vec::new();

    let mut key_pos = 0;
    for i in 0..img.width() {
        for j in 0..img.height() {
            let pix = img.get_pixel(i, j);

            for k in 0..3 {
                match key_bits[key_pos % key_bits.len()] {
                    0 => {
                        msg_bits.push(pix[k] & 1);
                    },
                    1 => {
                        msg_bits.push((pix[k] & (1 << 1)) / 2);
                        msg_bits.push(pix[k] & 1);
                    },
                    _ => panic!()
                }
                key_pos += 1;
            }
        }
    }
    let msg = bitvec_to_str(msg_bits);
    Ok(cut_str_eom(msg, EOM_IDENTIFIER))
    
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

fn bitvec_to_str(bitvec: BitVec) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..bitvec.len() {
        if i % 8 == 0 {
            bytes.push(0);
        }
        *bytes.last_mut().unwrap() = (bytes.last().unwrap() << 1) | bitvec[i];
    }
    unsafe {
        let string = String::from_utf8_unchecked(bytes);
        string
    }
}

fn cut_str_eom(string: String, eom: &str) -> String {
    let i = string.find(eom);
    if i != None {
        string[..i.unwrap()].to_string()
    } else {
        string
    }
}
