use clap::builder::Str;

pub fn enc_caeser(input : String, s : i32) -> String{
    input.chars().map(|c| shift(c, s)).collect()
}

pub fn dec_caeser(input : String, s : i32) -> String {
    input.chars().map(|c| shift(c, -s)).collect()
}

fn shift(c : char, shift : i32) -> char {

    // no shift in case of characters like newline or space to preserve string structure
    if !c.is_ascii_alphabetic(){
        return c;
    }

    let shifted_char : char = (c as u32).wrapping_add(shift as u32) as u8 as char;
    
    // Ensure that the character stays within the alphabetic range (a-z, A-Z)
    if c.is_ascii_alphabetic() && !shifted_char.is_ascii_alphabetic() {
        let adjust : i32= if shift > 0 { 26 } else { -26 };
        let shifted: char = (c as u8 as i32).wrapping_add(adjust) as u8 as char;
        if c.is_ascii_uppercase() { shifted.to_ascii_uppercase() } else { shifted }
    } else {
        shifted_char
    }
}