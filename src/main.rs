use std::{fmt::Error, fs, io};

use language::{parser::Parser, tokenizer::tokenize};
mod language {
    pub mod parser;
    pub mod tokenizer;
}

fn get_file_data() -> io::Result<String> {
    fs::read_to_string("/Users/anirban/Documents/Code/pydb/src/language/example/script1.pydb")
}

fn main() {
    let file_contents = get_file_data();

    match file_contents {
        Ok(s) => {
            let tokens = tokenize(&s);
            let mut parser = Parser::new(&tokens);
            println!("Tokens :: {:?}\n", tokens);
            println!("{:#?}", parser.parse());
        }
        Err(err) => println!("Error: failed to read file: {:?}", err),
    }
}
