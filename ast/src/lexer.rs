// https://towardsdatascience.com/understanding-compilers-for-humans-version-2-157f0edb02dd
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
  Number(i64),
  Plus,
  Minus,
  Multiply,
  Division,
  OpenParen,
  CloseParen,
}

impl Token {
  fn from(val: char) -> Option<Token> {
    match val {
      '+' => Some(Token::Plus),
      '-' => Some(Token::Minus),
      '*' => Some(Token::Multiply),
      '/' => Some(Token::Division),
      '(' => Some(Token::OpenParen),
      ')' => Some(Token::CloseParen),
      _ => None,
    }
  }
}

pub fn lex(input: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let chars = input.chars().collect::<Vec<char>>();
  let input_len = chars.len();

  let mut i: usize = 0;
  while i < chars.len() {
    match Token::from(chars[i]) {
      Some(token) => tokens.push(token),
      None => {
        let ch = chars[i];
        if ch.is_digit(10) {
          let mut num: String = ch.to_string();
          i += 1;
          while i < input_len && chars[i].is_digit(10) {
            num.push(chars[i]);
            i += 1;
          }
          let number: i64 = num.parse().expect("Must be valid num");
          tokens.push(Token::Number(number));
          continue;
        }
      }
    }
    i += 1
  }
  tokens
}
