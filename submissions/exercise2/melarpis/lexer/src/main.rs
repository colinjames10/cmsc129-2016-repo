use std::*;
use std::io::prelude::*;
mod lexeme;
mod lexer;
use lexer::Lexer;

fn main() {
    if env::args().count() != 2 { println!("Usage: {} <filename.fl>", env::args().nth(0).unwrap()); }
    else if !env::args().last().unwrap().ends_with("fl") { println!("Please input a valid filename. Extension should be <.fl>"); }
    else {
        let filename = env::args().last().unwrap();
        let mut f = fs::File::open(&filename).expect("FILE INPUT ERROR");
        //let mut bytes = Vec::<u8>::new();
        //f.read_to_end(&mut bytes).unwrap();
        let mut code = String::new();
        f.read_to_string(&mut code).expect("FILE READING ERROR");
        println!("Source {}:", filename);
        println!("{}", code);
        let mut lexer = Lexer::new(code);
        let lexemes = lexer.analyze();
        println!("Lexemes:");
        for l in lexemes.clone() {
            println!("{:?}", l);
        }
    }
}

#[test]
fn test() {
    let mut f = fs::File::open("tests/test.fl").expect("FILE INPUT ERROR");
    let mut code = String::new();
    f.read_to_string(&mut code).expect("FILE READING ERROR");
    let mut lexer = Lexer::new(code);
    let lexemes = lexer.analyze();
    assert_eq!(lexemes.len(), 35);
}
