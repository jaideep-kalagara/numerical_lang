use std::process;

use numerical_lang::{cli::Cli, lexer::Lexer, span::Span};

fn main() {
    let cli = Cli::new();

    let input = cli.read_input();
    let mut lexer = Lexer::new(input.as_str());
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(e) => {
            let (line, column) = Span::line_column(&input, e.span().start);

            println!("Error at line {}, column {}: {}", line, column, e);
            process::exit(1)
        }
    };
    if cli.output_tokens {
        cli.output_tokens_file(tokens);
    }
}
