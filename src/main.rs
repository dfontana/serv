mod lexer;
mod parser;

fn main() {
    let input = "(3-2)*12";
    println!("input: {:?}", input);
    let tokens = lexer::lex(input);
    println!("tokens: {:?}", tokens);
    let tree = parser::parse(tokens);
    println!("tree: {:?}", tree);
}