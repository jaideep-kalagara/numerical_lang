use colored::Colorize;
use std::process;

use numerical_lang::{cli::Cli, lexer::Lexer, parser::Parser, span::Span};

fn main() {
    let cli = Cli::new();

    let input = cli.read_input();
    let mut lexer = Lexer::new(input.as_str());
    let tokens = match lexer.lex() {
        Ok(t) => t,
        Err(errors) => {
            for e in errors {
                let (line, column) = Span::line_column(&input, e.span().start);

                println!(
                    "{}",
                    format!("Error at line {}, column {}: {}", line, column, e).red()
                );
            }
            process::exit(1)
        }
    };
    if cli.output_tokens {
        cli.output_tokens_file(&tokens);
    }
    let mut parser = Parser::new(tokens);
    let expr = match parser.expr() {
        Ok(e) => e,
        Err(e) => {
            let (line, column) = Span::line_column(&input, e.span().start);
            println!(
                "{}",
                format!("Error at line {}, column {}: {}", line, column, e).red()
            );
            process::exit(1)
        }
    };
    if cli.output_ast {
        cli.output_ast_file(&expr);
    }
}
