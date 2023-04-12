//암호화 함수
fn encrypt(text: &str, shift: u8) -> String {

    // 결과값을 담을 result 변수 생성
    let mut result = String::new();

    // std::strs 에 있는 String 내장함수로 String.chars()를 호출하여 해당 String을 char배열로 리턴한다.
    // for in 구문을 이용하여 char 배열 안에 있는 각각 요소의 값을 쉬프트 시킨다.
    for c in text.chars() {
        // is_ascii_alphabetic()을 이용하여 해당 char의 값이 아스키 코드상에서 알바벳인지 판별이 가능하다.
        // 이를 통해 알파벳와 알파벳이 아님을 구분하여 처리가 가능하다.
        if c.is_ascii_alphabetic() {
            // 해당 알파벳이 대문자 인지 소문자 인지 구별한다.
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            // ascii 코드값에서 해당 알파벳 순서 추출.
            let offset = c as u8 - base;
            // 해당 알파벳에 순서에 shift값을 더해서 암호 생성.
            let cipher = ((offset + shift) % 26) + base;
            // result 변수에 push.
            result.push(cipher as char);
        } else {
            result.push(c);
        }
    }

    // result 값 리턴
    result
}

// 복호화 함수
fn decrypt(text: &str, shift: u8) -> String {
    // 복호화는 암호화의 방식과 일치하기 때문에, 별다른 로직없이 encrypt의 shift 매개변수의 값을 26 - shift로 지정해주면 된다.
    encrypt(text, 26 - shift)
}


fn main() {
    // text : 암호화/복호화 시킬 텍스트,  shift : 쉬프트 시킬 값
    let text = "hello world";
    let shift = 3;
    // 텍스트를 shiftr값에 맞추어 암호화, 복호화
    let encrypted = encrypt(text, shift);
    let decrypted = decrypt(&encrypted, shift);
    println!("Original: {}", text);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}