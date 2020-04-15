mod lexer;
mod parser;

fn main() {
  let input = "12-((3*12+4-5)-2)+4";
  println!("input: {:?}", input);
  let tokens = lexer::lex(input);
  println!("tokens: {:?}", tokens);
  let tree = parser::parse(tokens);
  println!("tree: {:?}", tree);
}
