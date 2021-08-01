/* UTILS */

// type for storing a bunch of bits
pub type BitVec = Vec<u8>;

// convert a string into a BitVec
// example: "hey" -> [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1]
pub fn str_to_bitvec(string: &str) -> BitVec {
    let mut bitvec: BitVec = Vec::new();

    // iterate over every byte of the string
    for byte in string.as_bytes() {
        // get each of the eight bits of a byte and push it seperatly
        for i in (0..8).rev() {
            bitvec.push((byte & (1 << i)) / (1 << i));
        }
    }
    bitvec
}

// convert a BitVec into a string
// example: [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1] -> "hey"
pub fn bitvec_to_str(bitvec: BitVec) -> String {
    let mut bytes: Vec<u8> = Vec::new();

    // iterate over every bit
    for i in 0..bitvec.len() {
        // every eighth bit, push a new empty byte
        if i % 8 == 0 {
            bytes.push(0);
        }

        // add bit to last pushed byte
        *bytes.last_mut().unwrap() = (bytes.last().unwrap() << 1) | bitvec[i];
    }

    // convert bytes into string and return it
    let string = String::from_utf8_lossy(&bytes).into_owned();
    string
}

// cut a string if a pattern (EOM) matches
pub fn cut_str_eom(string: String, eom: &str, include_eom: bool) -> String {
    let i = string.find(eom);
    if i != None {
        if include_eom {
            return string[..i.unwrap() + eom.len()].to_string();
        } else {
            return string[..i.unwrap()].to_string();
        }
    }
    string
}
