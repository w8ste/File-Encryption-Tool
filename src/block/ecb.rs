pub fn encrypt_ecb(key : &[u8], plaintext : &[u8]) -> String {
    let mut s = String::new();
    for (pt_byte, key_byte) in plaintext.iter().zip(key.iter().cycle()) {
        // XOR each byte of the plaintext with the corresponding byte of the key
        let encrypted_byte = pt_byte ^ key_byte;
        s.push((encrypted_byte.clone() + b'0') as char);
    }

    s
}

pub fn decrypt_ecb(key : &[u8], c : &[u8]) -> String {
    encrypt_ecb(key, c)
}

fn xor_block(mut block : Vec<u8>, key : &[u8]) -> Vec<u8> {
    for i in 0..block.len() {
        block[i] ^= key[i];
    }
    println!("{:?}", String::from_utf8(block.clone()));
    block
}