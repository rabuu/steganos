mod utils;

use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};
use utils::*;

/* ENCRYPTION FUNCTION */
pub fn encrypt(msg: &str, key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {

    // read image
    let mut img = ImageReader::open(input_path)?.decode()?;

    // get BitVecs of message and key
    let msg_bits: BitVec = str_to_bitvec(msg);
    let key_bits: BitVec = str_to_bitvec(key);

    // keep track of position inside the strings/BitVecs
    let mut key_pos = 0;
    let mut msg_pos = 0;

    // iterate through the pixels of the image
    for i in 0..img.width() {
        for j in 0..img.height() {

            // store the pixel
            let mut pix = img.get_pixel(i, j);

            // iterate through the RGB values of each pixel
            for rgb in 0..3 {

                // current key value == 0 -> manipulate last bit
                // current key value == 1 -> manipulate the last two bits
                match key_bits[key_pos % key_bits.len()] {
                    0 => {
                        pix[rgb] >>= 1; // bit shift to get rid of last bit
                        pix[rgb] = (pix[rgb] << 1) | msg_bits[msg_pos % msg_bits.len()]; // shift back and set last bit to msg_bit
                        msg_pos += 1; // increase position of current msg_bit to hide
                    },
                    1 => {
                        // same as above but with last two bits
                        pix[rgb] >>= 2;
                        pix[rgb] = ((pix[rgb] << 2) | msg_bits[msg_pos % msg_bits.len()] << 1) | msg_bits[(msg_pos + 1) % msg_bits.len()];
                        msg_pos += 2;
                    },
                    _ => panic!("BitVec contains something else than 0 or 1")
                }

                // save manipulated pixel in the image
                img.put_pixel(i, j, pix);

                key_pos += 1; // next round there is a new key value
            }
        }
    }

    // return the manipulated image
    Ok(img)

}

/* DECRYPTION FUNCTION */
pub fn decrypt(input_path: &str, key: &str, eom: &str, include_eom: bool) -> Result<String, ImageError> {

    // read image
    let img = ImageReader::open(input_path)?.decode()?;

    // get BitVec of key
    let key_bits = str_to_bitvec(key);

    // create an empty BitVec to store the decrypted message
    let mut msg_bits: BitVec = Vec::new();

    // keep track of key value
    let mut key_pos = 0;

    // iterate through the pixels of the image
    for i in 0..img.width() {
        for j in 0..img.height() {

            // store the pixel
            let pix = img.get_pixel(i, j);

            // iterate through the RGB values of each pixel
            for rgb in 0..3 {

                // current key value == 0 -> store last bit
                // current key value == 1 -> store last two bits
                match key_bits[key_pos % key_bits.len()] {
                    0 => {
                        msg_bits.push(pix[rgb] & 1); // push least significant bit
                    },
                    1 => {
                        msg_bits.push((pix[rgb] & (1 << 1)) / 2); // push second least significant bit
                        msg_bits.push(pix[rgb] & 1); // push least significant bit
                    },
                    _ => panic!()
                }
                key_pos += 1; // next round there is a new key value
            }
        }
    }

    // parse the stored bits to a string
    let msg = bitvec_to_str(msg_bits);

    // return the message (EOM cut)
    Ok(cut_str_eom(msg, eom, include_eom))
    
}
