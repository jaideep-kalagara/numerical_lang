use numerical_lang::lexer::Lexer;

fn main() {
    let to_be_lexed = "+-*/ =() ;hello if";

    let mut lexer = Lexer::new(to_be_lexed.to_string());
    let tokens = lexer.lex();
    for token in tokens {
        println!("{}", token)
    }
}
