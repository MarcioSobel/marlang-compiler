use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(isize),
    Return,
    Semicolon,
}

pub fn tokenize(contents: &String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = contents.chars().peekable();

    let mut buffer = String::new();
    while let Some(char) = chars.next() {
        let next_char = chars.peek();

        if !char.is_whitespace() {
            buffer.push(char);
        }

        if char.is_whitespace() || next_char == Some(&';') {
            let token =
                get_token_type(&buffer).expect(format!("Unexpected token: {buffer}").as_str());
            tokens.push(token);
            buffer.clear();
        }
    }

    println!("{:?}", tokens);
    Ok(tokens)
}

fn get_token_type(buffer: &String) -> Option<Token> {
    if is_buffer_number(&buffer) {
        let value: isize = buffer.trim().parse().unwrap();
        return Some(Token::Number(value));
    }

    match buffer.as_str() {
        "return" => Some(Token::Return),
        ";" => Some(Token::Semicolon),
        _ => None,
    }
}

#[inline]
fn is_buffer_number(buffer: &String) -> bool {
    buffer.trim().parse::<isize>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert!(is_buffer_number(&String::from("2")));
        assert!(!is_buffer_number(&String::from("a")));
    }

    #[test]
    fn token_number() {
        let buf = String::from("25");
        assert_eq!(
            Token::Number(25),
            get_token_type(&buf).expect("Token should parse to a number")
        );
    }

    #[test]
    fn token_return() {
        let buf = String::from("return");
        assert_eq!(
            Token::Return,
            get_token_type(&buf).expect("Token should parse to return")
        );
    }

    #[test]
    fn token_semicolon() {
        let buf = String::from(";");
        assert_eq!(
            Token::Semicolon,
            get_token_type(&buf).expect("Token should parse to semicolon")
        );
    }

    #[test]
    fn basic_lex() {
        let contents = String::from("return;\n");
        assert_eq!(
            vec![Token::Return, Token::Semicolon],
            tokenize(&contents).expect("should be able to lex contents")
        );
    }
}
