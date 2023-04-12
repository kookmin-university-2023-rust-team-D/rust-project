struct CaesarCipher {
    shift: i16,
}

impl CaesarCipher {
    fn new(shift: i16) -> CaesarCipher {
        CaesarCipher { shift }
    }

    fn transform(&self, text: &str, shift: i16) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = c as u8 - base;
                let shifted_offset = ((offset as i16 + shift + 26) % 26) as u8;
                let cipher = shifted_offset + base;
                result.push(cipher as char);
            } else {
                result.push(c);
            }
        }
        result
    }

    fn encrypt(&self, text: &str) -> String {
        self.transform(text, self.shift)
    }

    fn decrypt(&self, text: &str) -> String {
        self.transform(text, -self.shift)
    }
}

fn main() {
    let cipher = CaesarCipher::new(3);
    let text = "hello world";
    let encrypted = cipher.encrypt(text);
    let decrypted = cipher.decrypt(&encrypted);
    println!("Original: {}", text);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
