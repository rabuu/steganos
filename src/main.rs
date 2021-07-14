/* EXTERN CRATES */
extern crate image;

#[macro_use]
extern crate clap; // CLI framework

/* IMPORTS */
use clap::App;
use image::{DynamicImage, GenericImage, GenericImageView, ImageError, io::Reader as ImageReader};

/* TYPE DEFINITIONS */
type BitVec = Vec<u8>;

/* DEFAULTS */
const ENCRYPTED_OUTPUT_DEFAULT: &str = "./encrypted_image.png";
const EOM_DEFAULT: &str = "*[END]*";


/* MAIN FUNCTION */
fn main() {

    // load cli.yml (the clap/CLI configuration)
    let yaml = load_yaml!("cli.yml");
    let clap = App::from_yaml(yaml).get_matches();

    // encrypt subcommand
    if let Some(clap) = clap.subcommand_matches("encrypt") {
        // store cli args
        let message = clap.value_of("message").unwrap();
        let key = clap.value_of("key").unwrap();
        let image = clap.value_of("image").unwrap();
        let output = clap.value_of("output").unwrap_or(ENCRYPTED_OUTPUT_DEFAULT);

        // encrypt and save file
        let encrypted_img = hide_msg_in_img(message, key, image).unwrap();
        encrypted_img.save(output).unwrap();
    }
    // decrypt subcommand
    else if let Some(clap) = clap.subcommand_matches("decrypt") {
        // store cli args
        let image = clap.value_of("image").unwrap();
        let key = clap.value_of("key").unwrap();
        let eom = clap.value_of("eom").unwrap_or(EOM_DEFAULT);
        let include_eom = clap.is_present("include-eom");

        // decrypt and output message
        let decrypted_msg = extract_msg_from_img(image, key, eom, include_eom).unwrap();
        println!("{}", decrypted_msg);
    }
}


/* ENCRYPTION FUNCTION */
fn hide_msg_in_img(msg: &str, key: &str, input_path: &str) -> Result<DynamicImage, ImageError> {

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
            for k in 0..3 {

                // current key value == 0 -> manipulate last bit
                // current key value == 1 -> manipulate the last two bits
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
fn extract_msg_from_img(input_path: &str, key: &str, eom: &str, include_eom: bool) -> Result<String, ImageError> {

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
            for k in 0..3 {

                // current key value == 0 -> store last bit
                // current key value == 1 -> store last two bits
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
                key_pos += 1; // next round there is a new key value
            }
        }
    }

    // parse the stored bits to a string
    let msg = bitvec_to_str(msg_bits);

    // return the message (EOM cut)
    Ok(cut_str_eom(msg, eom, include_eom))
    
}


/* HELPER FUNCTIONS */

// convert a string into a BitVec
// example: "hey" -> [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1]
fn str_to_bitvec(string: &str) -> BitVec {
    let mut bitvec: BitVec = Vec::new();
    for byte in string.as_bytes() {
        for i in 0..8 {
            bitvec.push((byte & (1 << (7 - i))) / (1 << (7 - i)));
        }
    }
    bitvec
}

// convert a BitVec into a string
// example: [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1] -> "hey"
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

// cut a string if a pattern (EOM) matches
fn cut_str_eom(string: String, eom: &str, include_eom: bool) -> String {
    let i = string.find(eom);
    if i != None {
        if include_eom {
            string[..i.unwrap() + eom.len()].to_string()
        } else {
            string[..i.unwrap()].to_string()
        }
    } else {
        string
    }
}
