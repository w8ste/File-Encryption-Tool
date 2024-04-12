fn enc_ecb(key : &[u8], plaintext : &[u8]) -> String {
    let mut s = String::new();
    for (pt_byte, key_byte) in plaintext.iter().zip(key.iter().cycle()) {
        // XOR each byte of the plaintext with the corresponding byte of the key
        let encrypted_byte = pt_byte ^ key_byte;
        s.push((encrypted_byte.clone() + b'0') as char);
    }

    s
}
fn enc_block(clear_block: &String, key: &str) -> String {
    let mut enc_block = String::new();
    for (i, c) in clear_block.chars().enumerate() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        enc_block.push(char::from(c as u8 ^ key_char as u8));
    }
    enc_block
}

pub fn encrypt_ecb(m: &str, key: &str, block_length: usize) -> String {

    let mut enc = String::new();
    for i in (0..m.len()).step_by(block_length) {
        let mut block = &m[i..std::cmp::min(i + block_length, m.len())].to_string();
        if block.len() % block_length != 0 {
            let padded_block = &pad(&mut block.clone().as_bytes().to_vec(), block_length);
            enc += &enc_block(&padded_block, key);
        }
        else {
            enc += &enc_block(&block.clone(), key);
        }
    }
    enc
}
pub fn decrypt_ecb(c : &str, key: &str, block_length : usize) -> String {
    encrypt_ecb(c.clone(), key, block_length)
}

fn pad(data: &mut Vec<u8>, block_size: usize) -> String{
    let padding_len = block_size - (data.len() % block_size);
    let padding_byte = padding_len as u8;
    data.resize(data.len() + padding_len, padding_byte);
    data.iter().map(|&byte| byte as char).collect()

}

fn unpad(data: &mut Vec<u8>, length : usize) -> String{
    //data.truncate(data.len() - padding_len);
    println!("In unpad: {}", data.len());
    println!("In unpad (padding_len): {}", length);
    data.iter().map(|&byte| byte as char).collect()
}



fn xor_block(mut block : Vec<u8>, key : &[u8]) -> Vec<u8> {
    for i in 0..block.len() {
        block[i] ^= key[i];
    }
    println!("{:?}", String::from_utf8(block.clone()));
    block
}