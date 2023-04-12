// 구조체 선언문.
// 해당 구문을 이용하여 구조체에 받을 값을 선언한다.
struct CaesarCipher {
    shift: u8,
}

// impl 키워드를 이용해서 CaesarCipher 에 대한 메소드를 구현한다.
impl CaesarCipher {

    // 생성자 함수
    // 생성자를 new 키워드를 이용하여 구조체를 초기화 시킨다.
    fn new(shift: u8) -> CaesarCipher {
        CaesarCipher { shift }
    }

    // 암호화 함수
    // 이전 구문과 똑값은 코드.
    fn encrypt(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = c as u8 - base;
                let cipher = ((offset + self.shift) % 26) + base;
                result.push(cipher as char);
            } else {
                result.push(c);
            }
        }
        result
    }

    // 복호화 함수
    fn decrypt(&self, text: &str) -> String {
        self.encrypt(text)
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
