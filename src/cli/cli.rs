/* EXTERN CRATES */
#[macro_use]
extern crate clap; // CLI framework

/* IMPORTS */
use clap::App;

use std::fs::File;
use std::io::Write;


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
        let encrypted_img = steganos::encrypt(message, key, image).expect("Encryption failed");
        encrypted_img.save(output).expect("Saving image failed");
    }
    // decrypt subcommand
    else if let Some(clap) = clap.subcommand_matches("decrypt") {
        // store cli args
        let image = clap.value_of("image").unwrap();
        let key = clap.value_of("key").unwrap();
        let eom = clap.value_of("eom").unwrap_or(EOM_DEFAULT);
        let include_eom = clap.is_present("include-eom");

        // decrypt and output message
        let decrypted_msg = steganos::decrypt(image, key, eom, include_eom).unwrap();

        // if output location is given, write message to file; otherwise just print it out
        if clap.is_present("output") {
            let path = clap.value_of("output").unwrap();
            let mut file = File::create(path).expect("File creation failed");
            file.write_all(decrypted_msg.as_bytes())
                .expect("Writing to file failed");
        } else {
            println!("{}", decrypted_msg);
        }
    }
}
