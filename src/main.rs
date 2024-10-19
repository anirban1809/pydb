use language::{parser::Parser, tokenizer::tokenize};
mod language {
    pub mod parser;
    pub mod tokenizer;
}

fn main() {
    let s = r#"a ** 3"#;

    let tokens = tokenize(s);
    let mut parser = Parser::new(&tokens);
    println!("Tokens :: {:?}\n", tokens);
    println!("{:#?}", parser.parse());
}
