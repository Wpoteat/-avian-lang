mod lexer;
use std::env;
use std::fs::File;

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_path: String = String::from(&args[1]);
    crate::lexer::tokenizer();
}
