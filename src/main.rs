use numerical_lang::{cli::Cli, lexer::Lexer};

fn main() {
    let cli = Cli::new();

    let mut lexer = Lexer::new(cli.read_input());

    if cli.output_tokens {
        cli.output_tokens_file(&mut lexer);
    }
}
