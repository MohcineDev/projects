#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // let text = "Hello, ASCII!";
    // let ascii_bytes = text.as_bytes();

    let mut new_str = String::from("");

    for byte in original.as_bytes() {
        if *byte >= b'a' && *byte <= b'z' {
            let mir = byte - b'a';
            let mir_char = b'z' - mir;
            new_str.push(mir_char as char);
        } else if *byte >= b'A' && *byte <= b'Z' {
            let mir = byte - b'A';
            let mir_char = b'Z' - mir;
            new_str.push(mir_char as char);
        } else {
            let c = *byte as char;
            new_str.push(c);
        }
    }

    if new_str == ciphered.to_string() {
        Ok(())
    } else {
        let a = CipherError {
            expected: String::from(new_str),
        };
        Err(a)
    }
}
